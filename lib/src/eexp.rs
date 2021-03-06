use std::io::prelude::*;
use regex::{ Regex ,Captures };
use def::StrMap;
use std::env ;
use std ;

pub struct EExpress
{
    regex: Regex,
    data : StrMap,

}

impl EExpress
{
    pub fn new(data : StrMap ) -> EExpress
    {
        let regex = Regex::new(r"(\$\{([[:alnum:]]+)\})").unwrap();
        EExpress{ regex, data  }
    }
    pub fn from_env() -> EExpress
    {
        let mut data = StrMap::new() ;
        for (key, value) in env::vars() {
            data.insert(key,value) ;
        }
        EExpress::new(data)
    }
    pub fn evar_val<'a>(&'a self, key : &str) -> Option<&'a String>
    {
         self.data.get(key)
    }
    pub fn safe_evar_val(& self, key : &str) -> String
    {
         if let Some(val) = self.evar_val(key) 
         {
             return val.clone() ;
         }
         return format!("__NO[{}]__", key) ;
    }
    pub fn parse<T>(&self, content :T) -> String
        where String: std::convert::From<T> 
    {
        let strc = String::from(content);
        let fun  =  | caps: &Captures| { format!("{}", self.safe_evar_val(&caps[2])) } ;
        self.regex.replace_all( strc.as_str() ,&fun).to_string() 
    }
}

#[cfg(test)]
mod tests
{
    use super::* ;
    use pretty_env_logger ;
    #[test]
    pub fn regex_verif()
    {
        let re      = Regex::new(r"(\$\{([[:alnum:]]+)\})").unwrap();
        let fun     =  | caps: &Captures| { format!("{}", &caps[2]) } ;
        let newc    = re.replace_all( "${HOME}/bin",  &fun) ;
        assert_eq!("HOME/bin",newc) ;
        let newc    = re.replace_all( "${HOME}/bin/${USER}",  &fun) ;
        assert_eq!("HOME/bin/USER",newc) ;
        let newc    = re.replace_all( "${HOME/bin",  &fun) ;
        assert_eq!("${HOME/bin",newc) ;

        let newc    = re.replace_all( "{HOME}/bin",  &fun) ;
        assert_eq!("{HOME}/bin",newc) ;
    }
    #[test]
    pub fn evar_verif()
    {
        let data = map!( 
            "HOME" => "/home/rigger",
            "USER" => "rigger"
            );
        
        let ex = EExpress::new(data);
        assert_eq!(ex.parse("${HOME}/bin"),String::from("/home/rigger/bin"));
        assert!(ex.parse("${HOME}/bin") != String::from("/home/rigger1/bin"));
        assert_eq!(ex.parse("${HOME}/${USER}/bin"),String::from("/home/rigger/rigger/bin"));
        assert_eq!(ex.parse("${HOME2}"),String::from("__NO[HOME2]__"));
        assert_eq!(ex.parse(format!("HOME2")),String::from("HOME2"));
    }

}
