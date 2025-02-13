pub mod interceptor {
    use tonic::{
        service::{interceptor::InterceptedService, Interceptor},
        transport::Channel,
        Status,
    };
    use tracing::trace;

    pub type Intercepted = InterceptedService<Channel, MyInterceptor>;

    #[derive(Clone, Copy)]
    pub struct MyInterceptor;

    impl Interceptor for MyInterceptor {
        fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
            trace!("intercepting");

            Ok(request)
        }
    }
}
