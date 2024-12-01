async fn run() {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    let adapter = {
        let instances = wgpu::Instance::default();
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            log::info!("Available adapters:");
            for a in instances.enumerate_adapters(wgpu::Backends::all()) {
                log::info!("    {:?}", a.get_info())
            }
        }
        instances
        .request_adapter(&wgpu::RequestAdapterOptionsBase::default())
        .await
        .unwrap()   
    };

    log::info!("Selected adapter: {:?}", adapter.get_info());
}


fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::builder()
        .parse_default_env()
        .init();
        pollster::block_on(run());
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}