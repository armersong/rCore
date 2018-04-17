pub mod vga;
pub mod acpi;
pub mod apic;
pub mod mp;
pub mod serial;
pub mod pic;
pub mod console;

pub fn init<F>(mut page_map: F)
    where F: FnMut(usize) {
    // TODO Handle this temp page map.
    page_map(0); // EBDA
    for addr in (0xE0000 .. 0x100000).step_by(0x1000) {
        page_map(addr);
    }
    page_map(0x7fe1000); // RSDT

    let acpi = acpi::init().expect("Failed to init ACPI");
    debug!("{:?}", acpi);

    if cfg!(feature = "use_apic") {
        pic::disable();

        page_map(acpi.lapic_addr as usize);  // LAPIC
        page_map(0xFEC00000);  // IOAPIC

        apic::init(acpi.lapic_addr, acpi.ioapic_id);
    } else {
        pic::init();
    }
    serial::SERIAL.lock().init();
    console::init();
}