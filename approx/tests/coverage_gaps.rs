use approx::{AbsDiff, Relative, Ulps};

fn panic_message(payload: Box<dyn core::any::Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_owned(),
            Err(_) => "<non-string panic payload>".to_owned(),
        },
    }
}

#[test]
fn builder_types_apply_custom_thresholds() {
    assert!(!AbsDiff::default().eq(&1.0f64, &1.5f64));
    assert!(AbsDiff::default().epsilon(0.5).eq(&1.0f64, &1.5f64));
    assert!(AbsDiff::default().epsilon(0.5).ne(&1.0f64, &1.6f64));

    assert!(!Relative::default().eq(&1.0f64, &1.4f64));
    assert!(Relative::default().max_relative(0.5).eq(&1.0f64, &1.4f64));
    assert!(Relative::default().max_relative(0.5).epsilon(0.0).eq(&1.0f64, &1.4f64));

    assert!(!Ulps::default().eq(&1.0f64, &(1.0f64 + 1e-15)));
    assert!(Ulps::default().max_ulps(5).eq(&1.0f64, &(1.0f64 + 1e-15)));
    assert!(Ulps::default().max_ulps(5).epsilon(0.0).eq(&1.0f64, &(1.0f64 + 1e-15)));
}

#[test]
fn assertion_macros_report_selected_options() {
    let panic = std::panic::catch_unwind(|| {
        approx::assert_relative_eq!(1.0f64, 2.0f64, epsilon = 0.0, max_relative = 0.0);
    })
    .expect_err("assertion should panic");

    let message = panic_message(panic);
    assert!(message.contains("assert_relative_eq!(1.0f64, 2.0f64, epsilon = 0.0, max_relative = 0.0)"));
    assert!(message.contains("left  = 1.0"));
    assert!(message.contains("right = 2.0"));
}

#[cfg(feature = "derive")]
#[test]
fn derive_feature_reexports_work_from_approx() {
    #[derive(approx::AbsDiffEq, Debug, PartialEq)]
    struct AbsReading {
        value: f64,
        #[approx(equal)]
        channel: u8,
    }

    #[derive(approx::RelativeEq, Debug, PartialEq)]
    struct RelativeReading {
        value: f64,
        #[approx(equal)]
        channel: u8,
    }

    let abs_lhs = AbsReading {
        value: 100.0,
        channel: 7,
    };
    let abs_rhs = AbsReading {
        value: 100.05,
        channel: 7,
    };
    let abs_different_channel = AbsReading {
        value: 100.05,
        channel: 8,
    };

    let relative_lhs = RelativeReading {
        value: 100.0,
        channel: 7,
    };
    let relative_rhs = RelativeReading {
        value: 100.05,
        channel: 7,
    };
    let relative_different_channel = RelativeReading {
        value: 100.05,
        channel: 8,
    };

    approx::assert_abs_diff_eq!(abs_lhs, abs_rhs, epsilon = 0.1);
    approx::assert_abs_diff_ne!(abs_lhs, abs_different_channel, epsilon = 0.1);
    approx::assert_relative_eq!(relative_lhs, relative_rhs, epsilon = 0.0, max_relative = 0.001);
    approx::assert_relative_ne!(
        relative_lhs,
        relative_different_channel,
        epsilon = 0.0,
        max_relative = 0.001
    );
}
