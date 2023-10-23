import struct
class Metric:
    GPS_LAT = 0,
    GPS_LON = 1,
    GPS_SPEED = 2,
    GPS_COURSE = 3,
    GPS_ALTITUDE = 4,
    GPS_HOR_ERROR = 5,
    GPS_VELO_UNCERTAINTY = 6,
    GPS_VERT_UNCERTAINTY = 7,
    ACCELERATION_X = 8,
    ACCELERATION_Y = 9,
    ACCELERATION_Z = 10,
    GYRO_X = 11,
    GYRO_Y = 12,
    GYRO_Z = 13,
    ROTATION_X = 14,
    ROTATION_Y = 15,
    ROTATION_Z = 16,
    CPU_FREQ = 17,
    CPU_USAGE = 18,
    RAM_USAGE = 19,
    CPU_TEMP = 20,
    FILTER_LON = 21,
    FILTER_LAT = 22,
    FILTER_ALTITUDE = 23,
    FILTER_SPEED = 24,
    FILTER_ROT_X = 25,
    FILTER_ROT_Y = 26,
    FILTER_ROT_Z = 27,
    APMU_TEMP = 28,
    MPMU_TEMP = 29,
    MOTOR_CURRENT = 30,
    BATTERY_VOLTAGE = 31,
    FAN_1 = 32,
    FAN_2 = 33,
    FAN_3 = 34,
    FAN_4 = 35,
    SOLAR_POWER = 36,
    SOLAR_TEMP = 37,
    MOTOR_TEMP = 38,
    MOTOR_INVERTER_TEMP = 39,
    MOTOR_BATTERY_VOLTAGE = 40,
    MOTOR_RPM = 41,
    MOTOR_POWER = 42,
    MOTOR_STATE = 43,
    ECU_MOTOR_CURRENT = 44,
    MOTOR_THROTTLE_POS = 45,
    MAX_BATTERY_DISCHARGE_CURRENT = 46,
    MAX_BATTERY_RECHARGE_CURRENT = 47,
    GLOBAL_SOC = 48,
    ID_GLOBAL_SOC = 49,
    GLOBAL_IBMS_ALARM_STATE = 50,
    NUMBER_OF_CONNECTED_BMS = 51,
    POWERBUS_INFORMATION = 52,
    BAT_TMIN = 53,
    BAT_TMAX = 54,
    BAT_ID_TMIN = 55,
    BAT_ID_TMAX = 56,
    BAT_VMIN = 57,
    BAT_VMAX = 58,
    BAT_ID_VMIN = 59,
    BAT_ID_VMAX = 60,
    GLOBAL_BAT_CURRENT = 61,
    GLOBAL_CELL_MIN = 62,
    GLOBAL_CELL_MAX = 63,
    GLOBAL_CELL_MIN_ID = 64,
    GLOBAL_CELL_MAX_ID  = 65,
    BAT_1_AH_DISCHARGED = 66,
    BAT_1_SOC = 67,
    BAT_1_SOH = 68,
    BAT_1_I_BAT_I = 69,
    BAT_1_REMAINING_CAPACITY = 70,
    BAT_1_T0 = 71,
    BAT_1_T1 = 72,
    BAT_1_T2 = 73,
    BAT_1_MAJOR_ALERT_1 = 74,
    BAT_1_MAJOR_ALERT_2 = 75,
    BAT_1_MAJOR_ALERT_3 = 76,
    BAT_1_MINOR_ALERT = 77,
    BAT_2_AH_DISCHARGED = 78,
    BAT_2_SOC = 79,
    BAT_2_SOH = 80,
    BAT_2_I_BAT_I = 81,
    BAT_2_REMAINING_CAPACITY = 82,
    BAT_2_T0 = 83,
    BAT_2_T1 = 84,
    BAT_2_T2 = 85,
    BAT_2_MAJOR_ALERT_1 = 86,
    BAT_2_MAJOR_ALERT_2 = 87,
    BAT_2_MAJOR_ALERT_3 = 88,
    BAT_2_MINOR_ALERT = 89,
    BAT_3_AH_DISCHARGED = 90,
    BAT_3_SOC = 91,
    BAT_3_SOH = 92,
    BAT_3_I_BAT_I = 93,
    BAT_3_REMAINING_CAPACITY = 94,
    BAT_3_T0 = 95,
    BAT_3_T1 = 96,
    BAT_3_T2 = 97,
    BAT_3_MAJOR_ALERT_1 = 98,
    BAT_3_MAJOR_ALERT_2 = 99,
    BAT_3_MAJOR_ALERT_3 = 100,
    BAT_3_MINOR_ALERT = 101,
    MEM_USED_MB = 102,
    MEM_TOTAL = 103,
    SWAP_USED_MB = 104,
    SWAP_TOTAL = 105,
    SYSTEM_UPTIME = 106,
    CPU_USAGE_USER = 107,
    CPU_USAGE_SYSTEM = 108,
    TX_IN_PER_SEC = 109,
    TX_OUT_PER_SEC = 110,
    TX_QUEUE_COUNT = 111,
    CELLULAR_SIGNAL_QUALITY = 112,
    CELLULAR_NETWORK_MODE = 113

METRIC_TYPE_VALUES =  {value: name for name, value in vars(Metric).items() if name.isupper()}

def get_metric_val(id, val):
    if id == Metric.CELLULAR_NETWORK_MODE:
        return val.decode("utf-8")
    else:
        return struct.unpack('<f', val)[0]
