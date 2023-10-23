use std::fmt;
use std::convert::TryFrom;
    
#[derive(Debug)]
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

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl TryFrom<u32> for Metric {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            0 if 0 == Metric::GpsLat as i32 => Ok(Metric::GpsLat),
            1 if 1 == Metric::GpsLon as i32 => Ok(Metric::GpsLon),
            2 if 2 == Metric::GpsSpeed as i32 => Ok(Metric::GpsSpeed),
            3 if 3 == Metric::GpsCourse as i32 => Ok(Metric::GpsCourse),
            4 if 4 == Metric::GpsAltitude as i32 => Ok(Metric::GpsAltitude),
            5 if 5 == Metric::GpsHorError as i32 => Ok(Metric::GpsHorError),
            6 if 6 == Metric::GpsVeloUncertainty as i32 => Ok(Metric::GpsVeloUncertainty),
            7 if 7 == Metric::GpsVertUncertainty as i32 => Ok(Metric::GpsVertUncertainty),
            8 if 8 == Metric::AccelerationX as i32 => Ok(Metric::AccelerationX),
            9 if 9 == Metric::AccelerationY as i32 => Ok(Metric::AccelerationY),
            10 if 10 == Metric::AccelerationZ as i32 => Ok(Metric::AccelerationZ),
            11 if 11 == Metric::GyroX as i32 => Ok(Metric::GyroX),
            12 if 12 == Metric::GyroY as i32 => Ok(Metric::GyroY),
            13 if 13 == Metric::GyroZ as i32 => Ok(Metric::GyroZ),
            14 if 14 == Metric::RotationX as i32 => Ok(Metric::RotationX),
            15 if 15 == Metric::RotationY as i32 => Ok(Metric::RotationY),
            16 if 16 == Metric::RotationZ as i32 => Ok(Metric::RotationZ),
            17 if 17 == Metric::CpuFreq as i32 => Ok(Metric::CpuFreq),
            18 if 18 == Metric::CpuUsage as i32 => Ok(Metric::CpuUsage),
            19 if 19 == Metric::RamUsage as i32 => Ok(Metric::RamUsage),
            20 if 20 == Metric::CpuTemp as i32 => Ok(Metric::CpuTemp),
            21 if 21 == Metric::FilterLon as i32 => Ok(Metric::FilterLon),
            22 if 22 == Metric::FilterLat as i32 => Ok(Metric::FilterLat),
            23 if 23 == Metric::FilterAltitude as i32 => Ok(Metric::FilterAltitude),
            24 if 24 == Metric::FilterSpeed as i32 => Ok(Metric::FilterSpeed),
            25 if 25 == Metric::FilterRotX as i32 => Ok(Metric::FilterRotX),
            26 if 26 == Metric::FilterRotY as i32 => Ok(Metric::FilterRotY),
            27 if 27 == Metric::FilterRotZ as i32 => Ok(Metric::FilterRotZ),
            28 if 28 == Metric::ApmuTemp as i32 => Ok(Metric::ApmuTemp),
            29 if 29 == Metric::MpmuTemp as i32 => Ok(Metric::MpmuTemp),
            30 if 30 == Metric::MotorCurrent as i32 => Ok(Metric::MotorCurrent),
            31 if 31 == Metric::BatteryVoltage as i32 => Ok(Metric::BatteryVoltage),
            32 if 32 == Metric::Fan1 as i32 => Ok(Metric::Fan1),
            33 if 33 == Metric::Fan2 as i32 => Ok(Metric::Fan2),
            34 if 34 == Metric::Fan3 as i32 => Ok(Metric::Fan3),
            35 if 35 == Metric::Fan4 as i32 => Ok(Metric::Fan4),
            36 if 36 == Metric::SolarPower as i32 => Ok(Metric::SolarPower),
            37 if 37 == Metric::SolarTemp as i32 => Ok(Metric::SolarTemp),
            38 if 38 == Metric::MotorTemp as i32 => Ok(Metric::MotorTemp),
            39 if 39 == Metric::MotorInverterTemp as i32 => Ok(Metric::MotorInverterTemp),
            40 if 40 == Metric::MotorBatteryVoltage as i32 => Ok(Metric::MotorBatteryVoltage),
            41 if 41 == Metric::MotorRpm as i32 => Ok(Metric::MotorRpm),
            42 if 42 == Metric::MotorPower as i32 => Ok(Metric::MotorPower),
            43 if 43 == Metric::MotorState as i32 => Ok(Metric::MotorState),
            44 if 44 == Metric::EcuMotorCurrent as i32 => Ok(Metric::EcuMotorCurrent),
            45 if 45 == Metric::MotorThrottlePos as i32 => Ok(Metric::MotorThrottlePos),
            46 if 46 == Metric::MaxBatteryDischargeCurrent as i32 => Ok(Metric::MaxBatteryDischargeCurrent),
            47 if 47 == Metric::MaxBatteryRechargeCurrent as i32 => Ok(Metric::MaxBatteryRechargeCurrent),
            48 if 48 == Metric::GlobalSoc as i32 => Ok(Metric::GlobalSoc),
            49 if 49 == Metric::IdGlobalSoc as i32 => Ok(Metric::IdGlobalSoc),
            50 if 50 == Metric::GlobalIbmsAlarmState as i32 => Ok(Metric::GlobalIbmsAlarmState),
            51 if 51 == Metric::NumberOfConnectedBms as i32 => Ok(Metric::NumberOfConnectedBms),
            52 if 52 == Metric::PowerbusInformation as i32 => Ok(Metric::PowerbusInformation),
            53 if 53 == Metric::BatTmin as i32 => Ok(Metric::BatTmin),
            54 if 54 == Metric::BatTmax as i32 => Ok(Metric::BatTmax),
            55 if 55 == Metric::BatIdTmin as i32 => Ok(Metric::BatIdTmin),
            56 if 56 == Metric::BatIdTmax as i32 => Ok(Metric::BatIdTmax),
            57 if 57 == Metric::BatVmin as i32 => Ok(Metric::BatVmin),
            58 if 58 == Metric::BatVmax as i32 => Ok(Metric::BatVmax),
            59 if 59 == Metric::BatIdVmin as i32 => Ok(Metric::BatIdVmin),
            60 if 60 == Metric::BatIdVmax as i32 => Ok(Metric::BatIdVmax),
            61 if 61 == Metric::GlobalBatCurrent as i32 => Ok(Metric::GlobalBatCurrent),
            62 if 62 == Metric::GlobalCellMin as i32 => Ok(Metric::GlobalCellMin),
            63 if 63 == Metric::GlobalCellMax as i32 => Ok(Metric::GlobalCellMax),
            64 if 64 == Metric::GlobalCellMinId as i32 => Ok(Metric::GlobalCellMinId),
            65 if 65 == Metric::GlobalCellMaxId  as i32 => Ok(Metric::GlobalCellMaxId ),
            66 if 66 == Metric::Bat1AhDischarged as i32 => Ok(Metric::Bat1AhDischarged),
            67 if 67 == Metric::Bat1Soc as i32 => Ok(Metric::Bat1Soc),
            68 if 68 == Metric::Bat1Soh as i32 => Ok(Metric::Bat1Soh),
            69 if 69 == Metric::Bat1IBatI as i32 => Ok(Metric::Bat1IBatI),
            70 if 70 == Metric::Bat1RemainingCapacity as i32 => Ok(Metric::Bat1RemainingCapacity),
            71 if 71 == Metric::Bat1T0 as i32 => Ok(Metric::Bat1T0),
            72 if 72 == Metric::Bat1T1 as i32 => Ok(Metric::Bat1T1),
            73 if 73 == Metric::Bat1T2 as i32 => Ok(Metric::Bat1T2),
            74 if 74 == Metric::Bat1MajorAlert1 as i32 => Ok(Metric::Bat1MajorAlert1),
            75 if 75 == Metric::Bat1MajorAlert2 as i32 => Ok(Metric::Bat1MajorAlert2),
            76 if 76 == Metric::Bat1MajorAlert3 as i32 => Ok(Metric::Bat1MajorAlert3),
            77 if 77 == Metric::Bat1MinorAlert as i32 => Ok(Metric::Bat1MinorAlert),
            78 if 78 == Metric::Bat2AhDischarged as i32 => Ok(Metric::Bat2AhDischarged),
            79 if 79 == Metric::Bat2Soc as i32 => Ok(Metric::Bat2Soc),
            80 if 80 == Metric::Bat2Soh as i32 => Ok(Metric::Bat2Soh),
            81 if 81 == Metric::Bat2IBatI as i32 => Ok(Metric::Bat2IBatI),
            82 if 82 == Metric::Bat2RemainingCapacity as i32 => Ok(Metric::Bat2RemainingCapacity),
            83 if 83 == Metric::Bat2T0 as i32 => Ok(Metric::Bat2T0),
            84 if 84 == Metric::Bat2T1 as i32 => Ok(Metric::Bat2T1),
            85 if 85 == Metric::Bat2T2 as i32 => Ok(Metric::Bat2T2),
            86 if 86 == Metric::Bat2MajorAlert1 as i32 => Ok(Metric::Bat2MajorAlert1),
            87 if 87 == Metric::Bat2MajorAlert2 as i32 => Ok(Metric::Bat2MajorAlert2),
            88 if 88 == Metric::Bat2MajorAlert3 as i32 => Ok(Metric::Bat2MajorAlert3),
            89 if 89 == Metric::Bat2MinorAlert as i32 => Ok(Metric::Bat2MinorAlert),
            90 if 90 == Metric::Bat3AhDischarged as i32 => Ok(Metric::Bat3AhDischarged),
            91 if 91 == Metric::Bat3Soc as i32 => Ok(Metric::Bat3Soc),
            92 if 92 == Metric::Bat3Soh as i32 => Ok(Metric::Bat3Soh),
            93 if 93 == Metric::Bat3IBatI as i32 => Ok(Metric::Bat3IBatI),
            94 if 94 == Metric::Bat3RemainingCapacity as i32 => Ok(Metric::Bat3RemainingCapacity),
            95 if 95 == Metric::Bat3T0 as i32 => Ok(Metric::Bat3T0),
            96 if 96 == Metric::Bat3T1 as i32 => Ok(Metric::Bat3T1),
            97 if 97 == Metric::Bat3T2 as i32 => Ok(Metric::Bat3T2),
            98 if 98 == Metric::Bat3MajorAlert1 as i32 => Ok(Metric::Bat3MajorAlert1),
            99 if 99 == Metric::Bat3MajorAlert2 as i32 => Ok(Metric::Bat3MajorAlert2),
            100 if 100 == Metric::Bat3MajorAlert3 as i32 => Ok(Metric::Bat3MajorAlert3),
            101 if 101 == Metric::Bat3MinorAlert as i32 => Ok(Metric::Bat3MinorAlert),
            102 if 102 == Metric::MemUsedMb as i32 => Ok(Metric::MemUsedMb),
            103 if 103 == Metric::MemTotal as i32 => Ok(Metric::MemTotal),
            104 if 104 == Metric::SwapUsedMb as i32 => Ok(Metric::SwapUsedMb),
            105 if 105 == Metric::SwapTotal as i32 => Ok(Metric::SwapTotal),
            106 if 106 == Metric::SystemUptime as i32 => Ok(Metric::SystemUptime),
            107 if 107 == Metric::CpuUsageUser as i32 => Ok(Metric::CpuUsageUser),
            108 if 108 == Metric::CpuUsageSystem as i32 => Ok(Metric::CpuUsageSystem),
            109 if 109 == Metric::TxInPerSec as i32 => Ok(Metric::TxInPerSec),
            110 if 110 == Metric::TxOutPerSec as i32 => Ok(Metric::TxOutPerSec),
            111 if 111 == Metric::TxQueueCount as i32 => Ok(Metric::TxQueueCount),
            112 if 112 == Metric::CellularSignalQuality as i32 => Ok(Metric::CellularSignalQuality),
            113 if 113 == Metric::CellularNetworkMode as i32 => Ok(Metric::CellularNetworkMode),
            _ => Err(()),
        }
    }
}

pub fn transform_metric_val(id: Metric, value: Vec<u8>) -> String {
    match id {
        Metric::CellularNetworkMode => String::from_utf8(value).unwrap(),
        _ => f32::from_be_bytes(value[0..4].try_into().unwrap()).to_string()
    }
}
