use std::fmt;
use std::convert::TryFrom;
    
#[derive(Debug, Clone)]
pub enum Metric {
    GpsLat = 0,
    GpsLon = 1,
    GpsSpeed = 2,
    GpsCourse = 3,
    GpsAltitude = 4,
    GpsHorError = 5,
    GpsVeloUncertainty = 6,
    GpsVertUncertainty = 7,
    AccelerationX = 8,
    AccelerationY = 9,
    AccelerationZ = 10,
    GyroX = 11,
    GyroY = 12,
    GyroZ = 13,
    RotationX = 14,
    RotationY = 15,
    RotationZ = 16,
    CpuFreq = 17,
    CpuUsage = 18,
    RamUsage = 19,
    CpuTemp = 20,
    FilterLon = 21,
    FilterLat = 22,
    FilterAltitude = 23,
    FilterSpeed = 24,
    FilterRotX = 25,
    FilterRotY = 26,
    FilterRotZ = 27,
    ApmuTemp = 28,
    MpmuTemp = 29,
    MotorCurrent = 30,
    BatteryVoltage = 31,
    Fan1 = 32,
    Fan2 = 33,
    Fan3 = 34,
    Fan4 = 35,
    SolarPower = 36,
    SolarTemp = 37,
    MotorTemp = 38,
    MotorInverterTemp = 39,
    MotorBatteryVoltage = 40,
    MotorRpm = 41,
    MotorPower = 42,
    MotorState = 43,
    EcuMotorCurrent = 44,
    MotorThrottlePos = 45,
    MaxBatteryDischargeCurrent = 46,
    MaxBatteryRechargeCurrent = 47,
    GlobalSoc = 48,
    IdGlobalSoc = 49,
    GlobalIbmsAlarmState = 50,
    NumberOfConnectedBms = 51,
    PowerbusInformation = 52,
    BatTmin = 53,
    BatTmax = 54,
    BatIdTmin = 55,
    BatIdTmax = 56,
    BatVmin = 57,
    BatVmax = 58,
    BatIdVmin = 59,
    BatIdVmax = 60,
    GlobalBatCurrent = 61,
    GlobalCellMin = 62,
    GlobalCellMax = 63,
    GlobalCellMinId = 64,
    GlobalCellMaxId  = 65,
    Bat1AhDischarged = 66,
    Bat1Soc = 67,
    Bat1Soh = 68,
    Bat1IBatI = 69,
    Bat1RemainingCapacity = 70,
    Bat1T0 = 71,
    Bat1T1 = 72,
    Bat1T2 = 73,
    Bat1MajorAlert1 = 74,
    Bat1MajorAlert2 = 75,
    Bat1MajorAlert3 = 76,
    Bat1MinorAlert = 77,
    Bat2AhDischarged = 78,
    Bat2Soc = 79,
    Bat2Soh = 80,
    Bat2IBatI = 81,
    Bat2RemainingCapacity = 82,
    Bat2T0 = 83,
    Bat2T1 = 84,
    Bat2T2 = 85,
    Bat2MajorAlert1 = 86,
    Bat2MajorAlert2 = 87,
    Bat2MajorAlert3 = 88,
    Bat2MinorAlert = 89,
    Bat3AhDischarged = 90,
    Bat3Soc = 91,
    Bat3Soh = 92,
    Bat3IBatI = 93,
    Bat3RemainingCapacity = 94,
    Bat3T0 = 95,
    Bat3T1 = 96,
    Bat3T2 = 97,
    Bat3MajorAlert1 = 98,
    Bat3MajorAlert2 = 99,
    Bat3MajorAlert3 = 100,
    Bat3MinorAlert = 101,
    MemUsedMb = 102,
    MemTotal = 103,
    SwapUsedMb = 104,
    SwapTotal = 105,
    SystemUptime = 106,
    CpuUsageUser = 107,
    CpuUsageSystem = 108,
    TxInPerSec = 109,
    TxOutPerSec = 110,
    TxQueueCount = 111,
    CellularSignalQuality = 112,
    CellularNetworkMode = 113
}

// string to enum
impl std::str::FromStr for Metric {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GpsLat" => Ok(Metric::GpsLat),
            "GpsLon" => Ok(Metric::GpsLon),
            "GpsSpeed" => Ok(Metric::GpsSpeed),
            "GpsCourse" => Ok(Metric::GpsCourse),
            "GpsAltitude" => Ok(Metric::GpsAltitude),
            "GpsHorError" => Ok(Metric::GpsHorError),
            "GpsVeloUncertainty" => Ok(Metric::GpsVeloUncertainty),
            "GpsVertUncertainty" => Ok(Metric::GpsVertUncertainty),
            "AccelerationX" => Ok(Metric::AccelerationX),
            "AccelerationY" => Ok(Metric::AccelerationY),
            "AccelerationZ" => Ok(Metric::AccelerationZ),
            "GyroX" => Ok(Metric::GyroX),
            "GyroY" => Ok(Metric::GyroY),
            "GyroZ" => Ok(Metric::GyroZ),
            "RotationX" => Ok(Metric::RotationX),
            "RotationY" => Ok(Metric::RotationY),
            "RotationZ" => Ok(Metric::RotationZ),
            "CpuFreq" => Ok(Metric::CpuFreq),
            "CpuUsage" => Ok(Metric::CpuUsage),
            "RamUsage" => Ok(Metric::RamUsage),
            "CpuTemp" => Ok(Metric::CpuTemp),
            "FilterLon" => Ok(Metric::FilterLon),
            "FilterLat" => Ok(Metric::FilterLat),
            "FilterAltitude" => Ok(Metric::FilterAltitude),
            "FilterSpeed" => Ok(Metric::FilterSpeed),
            "FilterRotX" => Ok(Metric::FilterRotX),
            "FilterRotY" => Ok(Metric::FilterRotY),
            "FilterRotZ" => Ok(Metric::FilterRotZ),
            "ApmuTemp" => Ok(Metric::ApmuTemp),
            "MpmuTemp" => Ok(Metric::MpmuTemp),
            "MotorCurrent" => Ok(Metric::MotorCurrent),
            "BatteryVoltage" => Ok(Metric::BatteryVoltage),
            "Fan1" => Ok(Metric::Fan1),
            "Fan2" => Ok(Metric::Fan2),
            "Fan3" => Ok(Metric::Fan3),
            "Fan4" => Ok(Metric::Fan4),
            "SolarPower" => Ok(Metric::SolarPower),
            "SolarTemp" => Ok(Metric::SolarTemp),
            "MotorTemp" => Ok(Metric::MotorTemp),
            "MotorInverterTemp" => Ok(Metric::MotorInverterTemp),
            "MotorBatteryVoltage" => Ok(Metric::MotorBatteryVoltage),
            "MotorRpm" => Ok(Metric::MotorRpm),
            "MotorPower" => Ok(Metric::MotorPower),
            "MotorState" => Ok(Metric::MotorState),
            "EcuMotorCurrent" => Ok(Metric::EcuMotorCurrent),
            "MotorThrottlePos" => Ok(Metric::MotorThrottlePos),
            "MaxBatteryDischargeCurrent" => Ok(Metric::MaxBatteryDischargeCurrent),
            "MaxBatteryRechargeCurrent" => Ok(Metric::MaxBatteryRechargeCurrent),
            "GlobalSoc" => Ok(Metric::GlobalSoc),
            "IdGlobalSoc" => Ok(Metric::IdGlobalSoc),
            "GlobalIbmsAlarmState" => Ok(Metric::GlobalIbmsAlarmState),
            "NumberOfConnectedBms" => Ok(Metric::NumberOfConnectedBms),
            "PowerbusInformation" => Ok(Metric::PowerbusInformation),
            "BatTmin" => Ok(Metric::BatTmin),
            "BatTmax" => Ok(Metric::BatTmax),
            "BatIdTmin" => Ok(Metric::BatIdTmin),
            "BatIdTmax" => Ok(Metric::BatIdTmax),
            "BatVmin" => Ok(Metric::BatVmin),
            "BatVmax" => Ok(Metric::BatVmax),
            "BatIdVmin" => Ok(Metric::BatIdVmin),
            "BatIdVmax" => Ok(Metric::BatIdVmax),
            "GlobalBatCurrent" => Ok(Metric::GlobalBatCurrent),
            "GlobalCellMin" => Ok(Metric::GlobalCellMin),
            "GlobalCellMax" => Ok(Metric::GlobalCellMax),
            "GlobalCellMinId" => Ok(Metric::GlobalCellMinId),
            "GlobalCellMaxId " => Ok(Metric::GlobalCellMaxId ),
            "Bat1AhDischarged" => Ok(Metric::Bat1AhDischarged),
            "Bat1Soc" => Ok(Metric::Bat1Soc),
            "Bat1Soh" => Ok(Metric::Bat1Soh),
            "Bat1IBatI" => Ok(Metric::Bat1IBatI),
            "Bat1RemainingCapacity" => Ok(Metric::Bat1RemainingCapacity),
            "Bat1T0" => Ok(Metric::Bat1T0),
            "Bat1T1" => Ok(Metric::Bat1T1),
            "Bat1T2" => Ok(Metric::Bat1T2),
            "Bat1MajorAlert1" => Ok(Metric::Bat1MajorAlert1),
            "Bat1MajorAlert2" => Ok(Metric::Bat1MajorAlert2),
            "Bat1MajorAlert3" => Ok(Metric::Bat1MajorAlert3),
            "Bat1MinorAlert" => Ok(Metric::Bat1MinorAlert),
            "Bat2AhDischarged" => Ok(Metric::Bat2AhDischarged),
            "Bat2Soc" => Ok(Metric::Bat2Soc),
            "Bat2Soh" => Ok(Metric::Bat2Soh),
            "Bat2IBatI" => Ok(Metric::Bat2IBatI),
            "Bat2RemainingCapacity" => Ok(Metric::Bat2RemainingCapacity),
            "Bat2T0" => Ok(Metric::Bat2T0),
            "Bat2T1" => Ok(Metric::Bat2T1),
            "Bat2T2" => Ok(Metric::Bat2T2),
            "Bat2MajorAlert1" => Ok(Metric::Bat2MajorAlert1),
            "Bat2MajorAlert2" => Ok(Metric::Bat2MajorAlert2),
            "Bat2MajorAlert3" => Ok(Metric::Bat2MajorAlert3),
            "Bat2MinorAlert" => Ok(Metric::Bat2MinorAlert),
            "Bat3AhDischarged" => Ok(Metric::Bat3AhDischarged),
            "Bat3Soc" => Ok(Metric::Bat3Soc),
            "Bat3Soh" => Ok(Metric::Bat3Soh),
            "Bat3IBatI" => Ok(Metric::Bat3IBatI),
            "Bat3RemainingCapacity" => Ok(Metric::Bat3RemainingCapacity),
            "Bat3T0" => Ok(Metric::Bat3T0),
            "Bat3T1" => Ok(Metric::Bat3T1),
            "Bat3T2" => Ok(Metric::Bat3T2),
            "Bat3MajorAlert1" => Ok(Metric::Bat3MajorAlert1),
            "Bat3MajorAlert2" => Ok(Metric::Bat3MajorAlert2),
            "Bat3MajorAlert3" => Ok(Metric::Bat3MajorAlert3),
            "Bat3MinorAlert" => Ok(Metric::Bat3MinorAlert),
            "MemUsedMb" => Ok(Metric::MemUsedMb),
            "MemTotal" => Ok(Metric::MemTotal),
            "SwapUsedMb" => Ok(Metric::SwapUsedMb),
            "SwapTotal" => Ok(Metric::SwapTotal),
            "SystemUptime" => Ok(Metric::SystemUptime),
            "CpuUsageUser" => Ok(Metric::CpuUsageUser),
            "CpuUsageSystem" => Ok(Metric::CpuUsageSystem),
            "TxInPerSec" => Ok(Metric::TxInPerSec),
            "TxOutPerSec" => Ok(Metric::TxOutPerSec),
            "TxQueueCount" => Ok(Metric::TxQueueCount),
            "CellularSignalQuality" => Ok(Metric::CellularSignalQuality),
            "CellularNetworkMode" => Ok(Metric::CellularNetworkMode),
            _ => Err(format!("'{}' is not a valid value for Metric", s)),
        }
    }
}

// enum to string representation
impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// u8 to enum
impl TryFrom<u8> for Metric {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Metric::GpsLat),
            1 => Ok(Metric::GpsLon),
            2 => Ok(Metric::GpsSpeed),
            3 => Ok(Metric::GpsCourse),
            4 => Ok(Metric::GpsAltitude),
            5 => Ok(Metric::GpsHorError),
            6 => Ok(Metric::GpsVeloUncertainty),
            7 => Ok(Metric::GpsVertUncertainty),
            8 => Ok(Metric::AccelerationX),
            9 => Ok(Metric::AccelerationY),
            10 => Ok(Metric::AccelerationZ),
            11 => Ok(Metric::GyroX),
            12 => Ok(Metric::GyroY),
            13 => Ok(Metric::GyroZ),
            14 => Ok(Metric::RotationX),
            15 => Ok(Metric::RotationY),
            16 => Ok(Metric::RotationZ),
            17 => Ok(Metric::CpuFreq),
            18 => Ok(Metric::CpuUsage),
            19 => Ok(Metric::RamUsage),
            20 => Ok(Metric::CpuTemp),
            21 => Ok(Metric::FilterLon),
            22 => Ok(Metric::FilterLat),
            23 => Ok(Metric::FilterAltitude),
            24 => Ok(Metric::FilterSpeed),
            25 => Ok(Metric::FilterRotX),
            26 => Ok(Metric::FilterRotY),
            27 => Ok(Metric::FilterRotZ),
            28 => Ok(Metric::ApmuTemp),
            29 => Ok(Metric::MpmuTemp),
            30 => Ok(Metric::MotorCurrent),
            31 => Ok(Metric::BatteryVoltage),
            32 => Ok(Metric::Fan1),
            33 => Ok(Metric::Fan2),
            34 => Ok(Metric::Fan3),
            35 => Ok(Metric::Fan4),
            36 => Ok(Metric::SolarPower),
            37 => Ok(Metric::SolarTemp),
            38 => Ok(Metric::MotorTemp),
            39 => Ok(Metric::MotorInverterTemp),
            40 => Ok(Metric::MotorBatteryVoltage),
            41 => Ok(Metric::MotorRpm),
            42 => Ok(Metric::MotorPower),
            43 => Ok(Metric::MotorState),
            44 => Ok(Metric::EcuMotorCurrent),
            45 => Ok(Metric::MotorThrottlePos),
            46 => Ok(Metric::MaxBatteryDischargeCurrent),
            47 => Ok(Metric::MaxBatteryRechargeCurrent),
            48 => Ok(Metric::GlobalSoc),
            49 => Ok(Metric::IdGlobalSoc),
            50 => Ok(Metric::GlobalIbmsAlarmState),
            51 => Ok(Metric::NumberOfConnectedBms),
            52 => Ok(Metric::PowerbusInformation),
            53 => Ok(Metric::BatTmin),
            54 => Ok(Metric::BatTmax),
            55 => Ok(Metric::BatIdTmin),
            56 => Ok(Metric::BatIdTmax),
            57 => Ok(Metric::BatVmin),
            58 => Ok(Metric::BatVmax),
            59 => Ok(Metric::BatIdVmin),
            60 => Ok(Metric::BatIdVmax),
            61 => Ok(Metric::GlobalBatCurrent),
            62 => Ok(Metric::GlobalCellMin),
            63 => Ok(Metric::GlobalCellMax),
            64 => Ok(Metric::GlobalCellMinId),
            65 => Ok(Metric::GlobalCellMaxId ),
            66 => Ok(Metric::Bat1AhDischarged),
            67 => Ok(Metric::Bat1Soc),
            68 => Ok(Metric::Bat1Soh),
            69 => Ok(Metric::Bat1IBatI),
            70 => Ok(Metric::Bat1RemainingCapacity),
            71 => Ok(Metric::Bat1T0),
            72 => Ok(Metric::Bat1T1),
            73 => Ok(Metric::Bat1T2),
            74 => Ok(Metric::Bat1MajorAlert1),
            75 => Ok(Metric::Bat1MajorAlert2),
            76 => Ok(Metric::Bat1MajorAlert3),
            77 => Ok(Metric::Bat1MinorAlert),
            78 => Ok(Metric::Bat2AhDischarged),
            79 => Ok(Metric::Bat2Soc),
            80 => Ok(Metric::Bat2Soh),
            81 => Ok(Metric::Bat2IBatI),
            82 => Ok(Metric::Bat2RemainingCapacity),
            83 => Ok(Metric::Bat2T0),
            84 => Ok(Metric::Bat2T1),
            85 => Ok(Metric::Bat2T2),
            86 => Ok(Metric::Bat2MajorAlert1),
            87 => Ok(Metric::Bat2MajorAlert2),
            88 => Ok(Metric::Bat2MajorAlert3),
            89 => Ok(Metric::Bat2MinorAlert),
            90 => Ok(Metric::Bat3AhDischarged),
            91 => Ok(Metric::Bat3Soc),
            92 => Ok(Metric::Bat3Soh),
            93 => Ok(Metric::Bat3IBatI),
            94 => Ok(Metric::Bat3RemainingCapacity),
            95 => Ok(Metric::Bat3T0),
            96 => Ok(Metric::Bat3T1),
            97 => Ok(Metric::Bat3T2),
            98 => Ok(Metric::Bat3MajorAlert1),
            99 => Ok(Metric::Bat3MajorAlert2),
            100 => Ok(Metric::Bat3MajorAlert3),
            101 => Ok(Metric::Bat3MinorAlert),
            102 => Ok(Metric::MemUsedMb),
            103 => Ok(Metric::MemTotal),
            104 => Ok(Metric::SwapUsedMb),
            105 => Ok(Metric::SwapTotal),
            106 => Ok(Metric::SystemUptime),
            107 => Ok(Metric::CpuUsageUser),
            108 => Ok(Metric::CpuUsageSystem),
            109 => Ok(Metric::TxInPerSec),
            110 => Ok(Metric::TxOutPerSec),
            111 => Ok(Metric::TxQueueCount),
            112 => Ok(Metric::CellularSignalQuality),
            113 => Ok(Metric::CellularNetworkMode),
            _ => Err(()),
        }
    }
}

// transform enum value to string representation
pub fn transform_metric_val(id: Metric, value: Vec<u8>) -> String {
    match id {
        Metric::CellularNetworkMode => String::from_utf8(value).unwrap(),
        _ => f32::from_ne_bytes(value[0..4].try_into().unwrap()).to_string()
    }
}