use chrono::Utc;
use once_cell::sync::OnceCell;
use std::sync::Once;
use std::time::Duration;
use tokio_proq::api::{ProqClient, ProqProtocol};
use tokio_proq::query_types::{ProqRulesType, ProqTargetStates};
use tokio_proq::result_types::ApiResult::ApiOk;

static CLIENT: OnceCell<ProqClient> = OnceCell::new();
static BARRIER: Once = Once::new();

fn client() -> &'static ProqClient {
    BARRIER.call_once(|| {
        let c = ProqClient::new_with_proto(
            "localhost:9090",
            ProqProtocol::HTTP,
            Some(Duration::from_secs(5)),
        )
        .unwrap();
        let _ = CLIENT.set(c);
    });

    CLIENT.get().unwrap()
}

#[test]
fn proq_instant_query() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().instant_query("up", None).await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_range_query() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let end = Utc::now();
        let start = Some(end - chrono::Duration::minutes(1));
        let step = Some(Duration::from_secs_f64(1.5));

        let x = match client()
            .range_query("up", start, Some(end), step)
            .await
            .unwrap()
        {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_series() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let end = Utc::now();
        let start = Some(end - chrono::Duration::hours(1));

        let selectors = vec!["up", "process_start_time_seconds{job=\"prometheus\"}"];

        let x = match client().series(selectors, start, Some(end)).await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_labels() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().label_names().await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_label_values() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let query_label = "version";

        let x = match client().label_values(query_label).await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_targets() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().targets().await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_targets_with_state() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let state_filer = ProqTargetStates::ACTIVE;

        let x = match client().targets_with_state(state_filer).await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_rules() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().rules().await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_rules_with_type() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let rule_type = ProqRulesType::ALERT;

        let x = match client().rules_with_type(rule_type).await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_alerts() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().alerts().await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_alert_manager() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().alert_managers().await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_config() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().config().await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}

#[test]
fn proq_flags() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let x = match client().flags().await.unwrap() {
            ApiOk(r) => {
                dbg!(r);
                true
            }
            e => {
                dbg!(e);
                false
            }
        };

        assert!(x)
    });
}
