use log::info;
use samp::amx::Amx;
use samp::initialize_plugin;
use samp::plugin::SampPlugin;
use std::io;

mod logger;

struct Plugin {}

impl SampPlugin for Plugin {
    // this function executed when samp server loads your plugin
    fn on_load(&mut self) {
        info!("Loaded");
    }

    fn on_unload(&mut self) {
        info!("Unloaded");
    }

    fn on_amx_unload(&mut self, _unloaded_amx: &Amx) {
        // empty
    }

    fn process_tick(&mut self) {
        // empty
    }
}

initialize_plugin!(
    natives: [
        Plugin::native_log,
    ],
    {
        samp::plugin::enable_process_tick();
        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("\t[SAMP LOGGER] {}: {}", record.level().to_string().to_lowercase(), message))
            })
             .chain(
                fern::Dispatch::new()
                    .level(log::LevelFilter::Info)
                    .chain(io::stdout()),
            )
            .apply();

        return Plugin{};
    }
);
