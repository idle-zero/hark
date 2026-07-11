use hark_core::model::{CpuInfo, DiskInfo, MemoryInfo, NetworkInfo, ProcessInfo, SystemSnapshot};
use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Length};

pub fn render<Message: 'static>(snapshot: &SystemSnapshot) -> Element<'_, Message> {
    let header = text("Hark").size(28);

    let content = column![
        header,
        cpu_view(&snapshot.cpu),
        memory_view(&snapshot.memory),
        disks_view(&snapshot.disks),
        networks_view(&snapshot.networks),
        processes_view(&snapshot.processes),
    ]
    .spacing(16)
    .padding(20);

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn cpu_view<Message: 'static>(cpu: &CpuInfo) -> Element<'_, Message> {
    let mut cores = column![text(format!("Global: {:.1}%", cpu.global_usage)).size(18)].spacing(4);

    for (i, usage) in cpu.cores.iter().enumerate() {
        cores = cores.push(text(format!("Core {}: {:.1}%", i, usage)).size(14));
    }

    column![text("CPU").size(20), cores].spacing(8).into()
}

fn memory_view<Message: 'static>(memory: &MemoryInfo) -> Element<'_, Message> {
    let used_mb = memory.used_bytes / 1024 / 1024;
    let total_mb = memory.total_bytes / 1024 / 1024;

    column![
        text("Memory").size(20),
        text(format!("{} MB / {} MB", used_mb, total_mb)).size(18),
    ]
    .spacing(8)
    .into()
}

fn disks_view<Message: 'static>(disks: &[DiskInfo]) -> Element<'_, Message> {
    let mut list = column![text("Disks").size(20)].spacing(4);

    for d in disks {
        let used_gb = d.used_bytes / 1024 / 1024 / 1024;
        let total_gb = d.total_bytes / 1024 / 1024 / 1024;
        list = list.push(text(format!("{}: {} GB / {} GB", d.name, used_gb, total_gb)).size(14));
    }

    column![list].spacing(8).into()
}

fn networks_view<Message: 'static>(networks: &[NetworkInfo]) -> Element<'_, Message> {
    let mut list = column![text("Network").size(20)].spacing(4);

    for n in networks {
        list = list.push(
            text(format!(
                "{}: ↓{} MB ↑{} MB",
                n.name,
                n.received_bytes / 1024 / 1024,
                n.transmitted_bytes / 1024 / 1024
            ))
            .size(14),
        );
    }

    column![list].spacing(8).into()
}

fn processes_view<Message: 'static>(processes: &[ProcessInfo]) -> Element<'_, Message> {
    let mut list = column![text("Top processes by CPU").size(20)].spacing(4);

    for p in processes {
        let line = row![
            text(format!("{}", p.pid)).width(Length::Fixed(80.0)),
            text(&p.name).width(Length::Fill),
            text(format!("{:.1}%", p.cpu_usage)).width(Length::Fixed(60.0)),
            text(format!("{} MB", p.memory_bytes / 1024 / 1024)).width(Length::Fixed(80.0)),
        ]
        .spacing(8);
        list = list.push(line);
    }

    column![list].spacing(8).into()
}
