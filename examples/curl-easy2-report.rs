use threescalers::{
    api_call::*,
    application::*,
    credentials::*,
    extensions::Extensions,
    http::{
        request::SetupRequest,
        Request,
    },
    service::*,
    transaction::Transaction,
    usage::Usage,
};

use threescalers::http::request::curl::BodyHandle;

use curl::easy::Easy2;

fn main() -> Result<(), threescalers::errors::Error> {
    let creds = Credentials::ServiceToken(ServiceToken::from("12[3]token"));
    let svc = Service::new("svc123", creds);
    let uks = ["userkey_1", "userkey_2", "userkey_3", "userkey 4", "userkey 5"];
    let apps = uks.iter()
                  .map(|uk| Application::from(UserKey::from(*uk)))
                  .collect::<Vec<_>>();

    println!("Apps: {:#?}", apps);

    let usages = [("metric11", 11),
                  ("metric12", 12),
                  ("metric21", 21),
                  ("metric22", 22),
                  ("metric31", 31),
                  ("metric32", 32),
                  ("metric41", 41),
                  ("metric42", 42),
                  ("metric51", 51),
                  ("metric52", 52)].chunks(2)
                                   .map(|metrics_and_values| Usage::new(metrics_and_values))
                                   .collect::<Vec<_>>();

    println!("Usages: {:#?}", usages);

    let ts = Default::default();

    let txns = apps.iter()
                   .zip(&usages)
                   .map(|(a, u)| Transaction::new(a, None, Some(u), Some(&ts)))
                   .collect::<Vec<_>>();

    let mut extensions = Extensions::new();
    extensions.insert("no_body", "1");
    extensions.insert("testing[=]", "0[=:=]0");
    let mut apicall = ApiCall::builder(&svc);
    let apicall = apicall.transactions(&txns)
                         .extensions(&extensions)
                         .kind(Kind::Report)
                         .build()?;
    let request = Request::from(&apicall);

    println!("apicall: {:#?}", apicall);
    println!("request: {:#?}", request);

    let _ = run_request(request);

    Ok(())
}

fn run_request(request: Request) -> Result<(), curl::Error> {
    let mut client = Easy2::new(BodyHandle::new());
    let _ = client.verbose(true).unwrap();
    client.setup_request(request, "https://echo-api.3scale.net");
    let result = exec_request(&client);
    show_response(client, result)
}

fn exec_request<H: std::fmt::Debug>(curlc: &Easy2<H>) -> Result<(), curl::Error> {
    println!("Client Easy2: {:#?}", curlc);
    curlc.perform()
}

// Not looking directly at the response but using the verbose mode.
fn show_response<H: std::fmt::Debug>(curlc: Easy2<H>,
                                     res: Result<(), curl::Error>)
                                     -> Result<(), curl::Error> {
    match res {
        Ok(_) => {
            println!("*** SUCCESS ***\n{:#?}", curlc);
            Ok(())
        }
        Err(e) => {
            println!("*** ERROR ***\n{:#?}", e);
            Err(e)
        }
    }
}
