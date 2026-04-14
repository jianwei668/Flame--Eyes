use crate::state::AppState;
use chrono::Utc;
use tokio::time::{interval, Duration};

async fn anonymize_hl7(raw: String) -> String {
    raw.replace("PID|1||", "PID|1||ANONYMIZED")
        .replace("NK1|", "NK1|ANONYMIZED|")
}

async fn mock_sm2_sign(data: &str) -> String {
    let end = data.len().min(20);
    format!("SM2_SIG_{}_{}", &data[..end], Utc::now().timestamp())
}

pub async fn run_pipeline(state: AppState) {
    let mut interval = interval(Duration::from_secs(2));
    loop {
        interval.tick().await;

        let raw = format!(
            "MSH|^~\\&|LIS|HIS|||{}||ORU^R01|1|P|2.3\nPID|1||ID9527||张三",
            Utc::now().format("%H%M%S")
        );

        {
            let mut s = state.stats.lock().await;
            s.captured_count += 1;
        }

        let cleaned = anonymize_hl7(raw).await;

        {
            let mut s = state.stats.lock().await;
            s.anonymized_count += 1;
        }

        let signed = mock_sm2_sign(&cleaned).await;

        {
            let mut s = state.stats.lock().await;
            s.sm2_signed_count += 1;
            s.pushed_to_cloud += 1;
            s.last_processed_at = Some(Utc::now().to_rfc3339());
        }

        tracing::info!("📡 流水线处理完成: {}", &signed[..signed.len().min(30)]);
    }
}