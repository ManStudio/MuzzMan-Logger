use muzzman_lib::{logger::Iam, prelude::*};
use std::io::Write;
use std::ops::Range;
use termcolor::{BufferWriter, ColorSpec, WriteColor};

#[module_link]
pub struct Logger;
impl TModule for Logger {
    fn init(&self, module_ref: MRef) -> Result<(), SessionError> {
        muzzman_lib::logger::init();
        muzzman_lib::logger::LOGGER_STATE
            .write()?
            .callbacks
            .push(Box::new(|from, record| {
                let mut bufwtr = BufferWriter::stdout(termcolor::ColorChoice::Always);
                let mut buffer = bufwtr.buffer();
                match record.level {
                    log::Level::Error => {
                        buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Red)));
                        write!(buffer, "ERROR: ");
                    }
                    log::Level::Warn => {
                        buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Yellow)));
                        write!(buffer, "Warn:  ");
                    }
                    log::Level::Info => {
                        buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::White)));
                        write!(buffer, "Info:  ");
                    }
                    log::Level::Debug => {
                        buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Blue)));
                        write!(buffer, "Debug: ");
                    }
                    log::Level::Trace => {
                        buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Green)));
                        write!(buffer, "Trace: ");
                    }
                };

                buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::White)));

                let from = match from {
                    Iam::Element { uid, id } => format!("{id:?}"),
                    Iam::Location { uid, id } => {
                        format!("{id:?}")
                    }
                    Iam::MuzzManLib => format!("MuzzManLib"),
                    Iam::Daemon => format!("Daemon"),
                };

                write!(buffer, "{from} ");

                buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Blue)));

                let time = <chrono::DateTime<chrono::Local>>::from(record.time);
                let time = time.format("%d/%m/%Y %H:%M");

                write!(buffer, "{time} ");

                buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::Ansi256(8))));

                if let Some(module_path) = &record.module_path {
                    write!(buffer, "{module_path} ");
                }

                if let Some(file) = &record.file {
                    if let Some(line) = record.line {
                        write!(buffer, "{file}:{line} ");
                    }
                }

                buffer.set_color(ColorSpec::new().set_fg(Some(termcolor::Color::White)));

                write!(buffer, "{}\n", record.log);

                bufwtr.print(&buffer);
            }));
        log::info!("Setup logger: {}!", self.get_version());
        Ok(())
    }

    fn get_name(&self) -> String {
        "Logger".into()
    }

    fn get_desc(&self) -> String {
        "This is storing every logs from elements and module in corresponding files".into()
    }

    fn get_uid(&self) -> UID {
        3
    }

    fn get_version(&self) -> String {
        "Logger: 1".into()
    }

    fn supported_versions(&self) -> std::ops::Range<u64> {
        1..2
    }

    fn init_settings(&self, data: &mut Values) -> Result<(), SessionError> {
        Ok(())
    }

    fn init_element_settings(&self, data: &mut Values) -> Result<(), SessionError> {
        Ok(())
    }

    fn init_location_settings(&self, data: &mut Values) -> Result<(), SessionError> {
        Ok(())
    }

    fn init_element(&self, element_row: ERow) -> Result<(), SessionError> {
        Ok(())
    }

    fn step_element(
        &self,
        element_row: ERow,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError> {
        Ok(())
    }

    fn accept_extension(&self, filename: &str) -> bool {
        false
    }

    fn accept_url(&self, url: String) -> bool {
        false
    }

    fn accepted_extensions(&self) -> Vec<String> {
        vec![]
    }

    fn accepted_protocols(&self) -> Vec<String> {
        vec![]
    }

    fn init_location(&self, location_ref: LRef) -> Result<(), SessionError> {
        Ok(())
    }

    fn step_location(
        &self,
        location_row: LRow,
        control_flow: &mut ControlFlow,
        storage: &mut Storage,
    ) -> Result<(), SessionError> {
        Ok(())
    }

    fn notify(&self, _ref: Ref, event: Event) -> Result<(), SessionError> {
        let Ref::Element(logger_element) = _ref else{return Ok(())};
        let session = logger_element.get_session()?;
        match event {
            Event::Element(element_id, event) => {
                println!("From Element: {element_id:?}, Event: {event:?}");
            }
            Event::Location(location_id, event) => {
                println!("From Location: {location_id:?}, Event: {event:?}");
            }
            Event::SessionEvent(event) => {
                println!("SessionEvent: {event:?}");
                match event {
                    SessionEvent::NewElement(element_id) => {
                        logger_element.subscribe(ID::Element(element_id));
                    }
                    SessionEvent::NewLocation(location_id) => {
                        logger_element.subscribe(ID::Location(location_id));
                    }
                    SessionEvent::NewModule(_) => {}
                    SessionEvent::DestroyedElement(element_id) => {
                        logger_element.unsubscribe(ID::Element(element_id));
                    }
                    SessionEvent::DestroyedLocation(location_id) => {
                        logger_element.unsubscribe(ID::Location(location_id));
                    }
                    SessionEvent::DestroyedModule(_) => {}
                    SessionEvent::ElementIdChanged(old_id, new_id) => {
                        logger_element.unsubscribe(ID::Element(old_id));
                        logger_element.subscribe(ID::Element(new_id));
                    }
                    SessionEvent::LocationIdChanged(old_id, new_id) => {
                        logger_element.unsubscribe(ID::Location(old_id));
                        logger_element.subscribe(ID::Location(new_id));
                    }
                    SessionEvent::ModuleIdChanged(_, _) => {}
                }
            }
        }
        Ok(())
    }

    fn c(&self) -> Box<dyn TModule> {
        Box::new(Logger)
    }
}
