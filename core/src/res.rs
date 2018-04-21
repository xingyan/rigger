
use err ;
use def::* ;
use model::* ;

pub trait SellDesp
{
    fn res_name(&self  ) ->String;
    fn res_info(&self  ) ->String;
    fn res_allow(&self , context : &mut Context  ) ->BoolR;
}
pub trait CallPlugin 
{
    fn res_before(&self , context : &mut Context ) ->BoolR;
    fn res_after(&self  , context : &mut Context ) ->BoolR;
}
pub trait StopBehavior
{
    fn res_stop(&self  , context : &mut Context  ) ->BoolR;
    fn res_clean(&self , context : &mut Context ) ->BoolR;
}
pub trait StartBehavior
{
    fn res_conf(&self  , context : &mut Context  ) ->BoolR;
    fn res_start(&self , context : &mut Context  ) ->BoolR;
    fn res_check(&self , context : &mut Context  ) ->BoolR;
}

impl<T> Res for T  
where T: SellDesp +  StartBehavior + StopBehavior + CallPlugin
{
    fn allow(&self,context : &mut Context) ->BoolR 
    {
        self.res_allow(context) 
    }
    fn conf(&self,context : &mut Context) ->BoolR 
    {
        self.res_before(context)? ;
        self.res_conf(context)? ;
        self.res_after(context)?;
        Ok(())
    }

    fn start(&self,context : &mut Context) ->BoolR 
    {
        self.res_before(context)? ;
        self.res_start(context)? ;
        self.res_after(context)?;
        Ok(())
    }

    fn stop(&self,context : &mut Context) ->BoolR 
    {
        self.res_before(context)? ;
        self.res_stop(context)? ;
        self.res_after(context)? ;
        Ok(())
    }
    fn check(&self,context : &mut Context) ->BoolR 
    {
        self.res_before(context)? ;
        self.res_check(context)? ;
        self.res_after(context)? ;
        Ok(())

    }
    fn clean(&self,context : &mut Context) ->BoolR 
    {
        self.res_before(context)? ;
        self.res_clean(context)? ;
        self.res_after(context)? ;
        Ok(())

    }
    fn info(&self) ->String
    {
        self.res_info()
    }
}

pub fn res_check<T>( res :&T) where T : Res
{
    let mut c = Context::new() ;
    assert!(res.conf(  &mut c).is_ok()) ;
    assert!(res.start( &mut c).is_ok());
    assert!(res.stop(  &mut c).is_ok()) ;
    assert!(res.clean( &mut c).is_ok()) ;
}

#[cfg(test)]
mod tests
{
    use super::* ;
    use pretty_env_logger ;
    struct StubRes {
    }
    impl StubRes {
        fn new() -> StubRes { StubRes{} }
    }
    //impl DefaultCtrl for StubRes {} 
    impl SellDesp  for StubRes
    {
        fn res_info(&self) -> String
        {
            format!("StubRes: {}","")

        }
        fn res_name(&self) -> String
        {
            format!("StubRes: {}","")

        }
        fn res_allow(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

        }

    }
    impl StartBehavior for StubRes
    {
        fn res_start(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

        }
        fn res_conf(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

        }
        fn res_check(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

        }
    }
    impl StopBehavior for StubRes 
    {
        fn res_stop(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

        }
        fn res_clean(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())

        }

    }
    impl CallPlugin for StubRes 
    {
        fn res_before(&self,_context : &mut Context) ->BoolR 
        {
            Ok(())
        }

        fn res_after(&self,_context : &mut Context) ->BoolR 
        {
            trace!("StubRes::res_after") ;
            Ok(())

        }
    }
    #[test]
    fn useres_stub()
    {

        let res = StubRes::new();
        res_check(&res) ;
    }
}
