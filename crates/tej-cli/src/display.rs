use comfy_table::{Cell, Color, Table};
use tej_core::SpeedTestResult;

pub fn print_results(result: &SpeedTestResult) {
    println!();

    let mut table = Table::new();
    table.set_header(vec![
        Cell::new("Metric").fg(Color::Cyan),
        Cell::new("Value").fg(Color::Cyan),
    ]);

    if let Some(ref loc) = result.server_location {
        table.add_row(vec!["Server", loc]);
    }

    if let Some(ref latency) = result.latency {
        table.add_row(vec![
            "Latency (avg)".to_string(),
            format!("{:.1} ms", latency.avg_ms),
        ]);
        table.add_row(vec![
            "Latency (min/max)".to_string(),
            format!("{:.1} / {:.1} ms", latency.min_ms, latency.max_ms),
        ]);
        table.add_row(vec![
            "Jitter".to_string(),
            format!("{:.1} ms", latency.jitter_ms),
        ]);
    }

    if let Some(ref dl) = result.download {
        table.add_row(vec!["Download".to_string(), format!("{:.2} Mbps", dl.mbps)]);
    }

    if let Some(ref ul) = result.upload {
        table.add_row(vec!["Upload".to_string(), format!("{:.2} Mbps", ul.mbps)]);
    }

    if let Some(loss) = result.packet_loss {
        table.add_row(vec!["Packet Loss".to_string(), format!("{:.1}%", loss)]);
    }

    println!("{table}");
}
