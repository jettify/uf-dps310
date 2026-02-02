use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};
use uf_dps3xx::{Config, DPS3xx, InitState, Register};

const ADDR: u8 = 0x77;

#[test]
fn test_new_dps3xx_defaults() {
    let expectations = [
        I2cTransaction::write_read(ADDR, vec![Register::PROD_ID.addr()], vec![0x10]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::PRS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_COEF_SRCE.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::TEMP_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::CFG_REG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![0x0E, 0xA5]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x96]),
        I2cTransaction::write(ADDR, vec![0x62, 0x02]),
        I2cTransaction::write(ADDR, vec![0x0E, 0x00]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
    ];

    let mut i2c = I2cMock::new(&expectations);
    let config = Config::new();
    let dps = DPS3xx::new(i2c.clone(), ADDR, &config).unwrap();
    let dps = dps.start_init().unwrap();
    let _dps = match dps.try_finish_init().unwrap() {
        InitState::Ready(dps) => dps,
        InitState::InProgress(_) => panic!("init not ready"),
    };
    i2c.done();
}

#[test]
fn test_read_calibration_coefficients() {
    let expectations = [
        I2cTransaction::write_read(ADDR, vec![Register::PROD_ID.addr()], vec![0x10]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::PRS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_COEF_SRCE.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::TEMP_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::CFG_REG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![0x0E, 0xA5]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x96]),
        I2cTransaction::write(ADDR, vec![0x62, 0x02]),
        I2cTransaction::write(ADDR, vec![0x0E, 0x00]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::COEFF_REG_1.addr()], vec![0; 18]),
    ];

    let mut i2c = I2cMock::new(&expectations);
    let config = Config::new();
    let dps = DPS3xx::new(i2c.clone(), ADDR, &config).unwrap();
    let dps = dps.start_init().unwrap();
    let dps = match dps.try_finish_init().unwrap() {
        InitState::Ready(dps) => dps,
        InitState::InProgress(_) => panic!("init not ready"),
    };

    let _dps = dps.read_calibration_coefficients().unwrap();
    i2c.done();
}

#[test]
fn test_trigger_measurement() {
    let expectations = [
        I2cTransaction::write_read(ADDR, vec![Register::PROD_ID.addr()], vec![0x10]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::PRS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_COEF_SRCE.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::TEMP_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::CFG_REG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![0x0E, 0xA5]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x96]),
        I2cTransaction::write(ADDR, vec![0x62, 0x02]),
        I2cTransaction::write(ADDR, vec![0x0E, 0x00]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
    ];

    let mut i2c = I2cMock::new(&expectations);
    let config = Config::new();
    let dps = DPS3xx::new(i2c.clone(), ADDR, &config).unwrap();
    let dps = dps.start_init().unwrap();
    let mut dps = match dps.try_finish_init().unwrap() {
        InitState::Ready(dps) => dps,
        InitState::InProgress(_) => panic!("init not ready"),
    };

    dps.trigger_measurement(true, false, false).unwrap();
    i2c.done();
}

#[test]
fn test_read_temp_calibrated() {
    let expectations = [
        I2cTransaction::write_read(ADDR, vec![Register::PROD_ID.addr()], vec![0x10]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::PRS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_COEF_SRCE.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::TEMP_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::CFG_REG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![0x0E, 0xA5]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x96]),
        I2cTransaction::write(ADDR, vec![0x62, 0x02]),
        I2cTransaction::write(ADDR, vec![0x0E, 0x00]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::COEFF_REG_1.addr()], vec![0; 18]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
    ];

    let mut i2c = I2cMock::new(&expectations);
    let config = Config::new();
    let dps = DPS3xx::new(i2c.clone(), ADDR, &config).unwrap();
    let dps = dps.start_init().unwrap();
    let dps = match dps.try_finish_init().unwrap() {
        InitState::Ready(dps) => dps,
        InitState::InProgress(_) => panic!("init not ready"),
    };
    let mut dps = dps.read_calibration_coefficients().unwrap();

    let _temp = dps.read_temp_calibrated().unwrap();
    i2c.done();
}

#[test]
fn test_status_and_ready_flags() {
    let expectations = [
        // Init
        I2cTransaction::write_read(ADDR, vec![Register::PROD_ID.addr()], vec![0x10]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::PRS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_COEF_SRCE.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::TEMP_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::CFG_REG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![0x0E, 0xA5]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x96]),
        I2cTransaction::write(ADDR, vec![0x62, 0x02]),
        I2cTransaction::write(ADDR, vec![0x0E, 0x00]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0xF0]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x80]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x40]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x10]),
    ];

    let mut i2c = I2cMock::new(&expectations);
    let config = Config::new();
    let dps = DPS3xx::new(i2c.clone(), ADDR, &config).unwrap();
    let dps = dps.start_init().unwrap();
    let mut dps = match dps.try_finish_init().unwrap() {
        InitState::Ready(dps) => dps,
        InitState::InProgress(_) => panic!("init not ready"),
    };

    assert_eq!(dps.read_status().unwrap(), 0xF0);
    assert!(dps.coef_ready().unwrap());
    assert!(dps.init_complete().unwrap());
    assert!(dps.temp_ready().unwrap());
    assert!(dps.pres_ready().unwrap());
    i2c.done();
}

#[test]
fn test_read_pressure_calibrated() {
    let expectations = [
        I2cTransaction::write_read(ADDR, vec![Register::PROD_ID.addr()], vec![0x10]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::PRS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_COEF_SRCE.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::TEMP_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::CFG_REG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![0x0E, 0xA5]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x96]),
        I2cTransaction::write(ADDR, vec![0x62, 0x02]),
        I2cTransaction::write(ADDR, vec![0x0E, 0x00]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::COEFF_REG_1.addr()], vec![0; 18]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::PSR_B2.addr()], vec![0x00, 0x04, 0x00]), // 1024
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x04, 0x00]), // 1024
    ];

    let mut i2c = I2cMock::new(&expectations);
    let config = Config::new();
    let dps = DPS3xx::new(i2c.clone(), ADDR, &config).unwrap();
    let dps = dps.start_init().unwrap();
    let dps = match dps.try_finish_init().unwrap() {
        InitState::Ready(dps) => dps,
        InitState::InProgress(_) => panic!("init not ready"),
    };
    let mut dps = dps.read_calibration_coefficients().unwrap();

    let pres = dps.read_pressure_calibrated().unwrap();
    // With all coeffs 0 and raw values 1024, scaled values 1024/524288 = 0.001953125
    // pres_cal = C00 + pres_scaled * (C10 + pres_scaled * (C20 + pres_scaled * C30)) + ...
    // = 0 + 0.00195... * (0 + ...) = 0
    assert_eq!(pres, 0.0);
    i2c.done();
}

#[test]
fn test_reset() {
    let expectations = [
        I2cTransaction::write_read(ADDR, vec![Register::PROD_ID.addr()], vec![0x10]),
        I2cTransaction::write_read(ADDR, vec![Register::PRS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::PRS_CFG.addr(), 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TEMP_CFG.addr()], vec![0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_COEF_SRCE.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::TEMP_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::CFG_REG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![0x0E, 0xA5]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x96]),
        I2cTransaction::write(ADDR, vec![0x62, 0x02]),
        I2cTransaction::write(ADDR, vec![0x0E, 0x00]),
        I2cTransaction::write(ADDR, vec![0x0F, 0x00]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x02]),
        I2cTransaction::write_read(ADDR, vec![Register::MEAS_CFG.addr()], vec![0x20]),
        I2cTransaction::write_read(ADDR, vec![Register::TMP_B2.addr()], vec![0x00, 0x00, 0x00]),
        I2cTransaction::write(ADDR, vec![Register::MEAS_CFG.addr(), 0x00]),
        I2cTransaction::write(ADDR, vec![Register::RESET.addr(), 0x89]),
    ];

    let mut i2c = I2cMock::new(&expectations);
    let config = Config::new();
    let dps = DPS3xx::new(i2c.clone(), ADDR, &config).unwrap();
    let dps = dps.start_init().unwrap();
    let dps = match dps.try_finish_init().unwrap() {
        InitState::Ready(dps) => dps,
        InitState::InProgress(_) => panic!("init not ready"),
    };

    let _dps = dps.reset().unwrap();
    i2c.done();
}
