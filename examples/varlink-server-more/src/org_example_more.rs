//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io;

use varlink;
use serde_json;
use varlink::CallTrait;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct State {
    #[serde(skip_serializing_if = "Option::is_none")] pub start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub progress: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub end: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct _PingReply {
    #[serde(skip_serializing_if = "Option::is_none")] pong: Option<String>,
}

impl varlink::VarlinkReply for _PingReply {}

#[derive(Serialize, Deserialize, Debug)]
struct _PingArgs {
    #[serde(skip_serializing_if = "Option::is_none")] ping: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct _TestMoreReply {
    #[serde(skip_serializing_if = "Option::is_none")] state: Option<State>,
}

impl varlink::VarlinkReply for _TestMoreReply {}

#[derive(Serialize, Deserialize, Debug)]
struct _TestMoreArgs {
    #[serde(skip_serializing_if = "Option::is_none")] n: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct _TestMoreErrorArgs {
    #[serde(skip_serializing_if = "Option::is_none")] reason: Option<String>,
}

pub trait _CallErr: varlink::CallTrait {
    fn reply_test_more_error(&mut self, reason: Option<String>) -> io::Result<()> {
        self.reply_struct(varlink::Reply::error(
            "org.example.more.TestMoreError".into(),
            Some(serde_json::to_value(_TestMoreErrorArgs { reason }).unwrap()),
        ))
    }
}

impl<'a> _CallErr for varlink::Call<'a> {}

pub trait _CallPing: _CallErr {
    fn reply(&mut self, pong: Option<String>) -> io::Result<()> {
        self.reply_struct(_PingReply { pong }.into())
    }
}

impl<'a> _CallPing for varlink::Call<'a> {}

pub trait _CallStopServing: _CallErr {
    fn reply(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> _CallStopServing for varlink::Call<'a> {}

pub trait _CallTestMethodNotImplemented: _CallErr {
    fn reply(&mut self) -> io::Result<()> {
        self.reply_struct(varlink::Reply::parameters(None))
    }
}

impl<'a> _CallTestMethodNotImplemented for varlink::Call<'a> {}

pub trait _CallTestMore: _CallErr {
    fn reply(&mut self, state: Option<State>) -> io::Result<()> {
        self.reply_struct(_TestMoreReply { state }.into())
    }
}

impl<'a> _CallTestMore for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn ping(&self, call: &mut _CallPing, ping: Option<String>) -> io::Result<()>;
    fn stop_serving(&self, call: &mut _CallStopServing) -> io::Result<()>;
    fn test_method_not_implemented(
        &self,
        call: &mut _CallTestMethodNotImplemented,
    ) -> io::Result<()>;
    fn test_more(&self, call: &mut _CallTestMore, n: Option<i64>) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"
 # Example service
interface org.example.more

# Enum, returning either start, progress or end
# progress: [0-100]
type State (
  start: bool,
  progress: int,
  end: bool
)

# Returns the same string
method Ping(ping: string) -> (pong: string)

# Dummy progress method
# n: number of progress steps
method TestMore(n: int) -> (state: State)

# Stop serving
method StopServing() -> ()

# Test for MethodNotImplemented
method TestMethodNotImplemented() ->()

# Something failed in TestMore
error TestMoreError (reason: string)

"#
    }

    fn get_name(&self) -> &'static str {
        "org.example.more"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        let method = req.method.clone();
        match method.as_ref() {
            "org.example.more.Ping" => {
                if let Some(args) = req.parameters.clone() {
                    let args: _PingArgs = serde_json::from_value(args)?;
                    return self.inner.ping(call as &mut _CallPing, args.ping);
                } else {
                    return call.reply_invalid_parameter(None);
                }
            }
            "org.example.more.StopServing" => {
                return self.inner.stop_serving(call as &mut _CallStopServing);
            }
            "org.example.more.TestMethodNotImplemented" => {
                return self.inner
                    .test_method_not_implemented(call as &mut _CallTestMethodNotImplemented);
            }
            "org.example.more.TestMore" => {
                if let Some(args) = req.parameters.clone() {
                    let args: _TestMoreArgs = serde_json::from_value(args)?;
                    return self.inner.test_more(call as &mut _CallTestMore, args.n);
                } else {
                    return call.reply_invalid_parameter(None);
                }
            }

            m => {
                let method: String = m.clone().into();
                return call.reply_method_not_found(Some(method));
            }
        }
    }
}
