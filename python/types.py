class MetricTypes:
    GPS_LAT = 0
    GPS_LON = 1
    GPS_SPEED = 2
    GPS_COURSE = 3
    GPS_ALTITUDE = 4
    GPS_HOR_ERROR = 5
    GPS_VELO_UNCERTAINTY = 6
    GPS_VERT_UNCERTAINTY = 7
    ACCELERATION_X = 8
    ACCELERATION_Y = 9
    ACCELERATION_Z = 10
    GYRO_X = 11
    GYRO_Y = 12
    GYRO_Z = 13
    ROTATION_X = 14
    ROTATION_Y = 15
    ROTATION_Z = 16
    CPU_FREQ = 17
    CPU_USAGE = 18
    RAM_USAGE = 19
    TX_QUEUE_COUNT = 20
    PENDING_TX = 21
    CPU_TEMP = 22
    FILTER_LON = 23
    FILTER_LAT = 24
    FILTER_ALTITUDE = 25
    FILTER_SPEED = 26
    FILTER_ROT_X = 27
    FILTER_ROT_Y = 28
    FILTER_ROT_Z = 29
    APMU_TEMP = 30
    MPMU_TEMP = 31
    MOTOR_CURRENT = 32
    BATTERY_VOLTAGE = 33
    FAN_1 = 34
    FAN_2 = 35
    FAN_3 = 36
    FAN_4 = 37
    SOLAR_POWER = 38
    SOLAR_TEMP = 39
    MOTOR_TEMP = 40
    MOTOR_INVERTER_TEMP = 41
    MOTOR_BATTERY_VOLTAGE = 42
    MOTOR_RPM = 43
    MOTOR_POWER = 44
    MOTOR_STATE = 45
    ECU_MOTOR_CURRENT = 46
    MOTOR_THROTTLE_POS = 47
    MAX_BATTERY_DISCHARGE_CURRENT = 48
    MAX_BATTERY_RECHARGE_CURRENT = 49
    GLOBAL_SOC = 50
    ID_GLOBAL_SOC = 51
    GLOBAL_IBMS_ALARM_STATE = 52
    NUMBER_OF_CONNECTED_BMS = 53
    POWERBUS_INFORMATION = 54
    BAT_TMIN = 55
    BAT_TMAX = 56
    BAT_ID_TMIN = 57
    BAT_ID_TMAX = 58
    BAT_VMIN = 59
    BAT_VMAX = 60
    BAT_ID_VMIN = 61
    BAT_ID_VMAX = 62
    GLOBAL_BAT_CURRENT = 63
    GLOBAL_CELL_MIN = 64
    GLOBAL_CELL_MAX = 65
    GLOBAL_CELL_MIN_ID = 66
    GLOBAL_CELL_MAX_ID  = 67
    BAT_1_AH_DISCHARGED = 68
    BAT_1_SOC = 69
    BAT_1_SOH = 70
    BAT_1_I_BAT_I = 71
    BAT_1_REMAINING_CAPACITY = 72
    BAT_1_T0 = 73
    BAT_1_T1 = 74
    BAT_1_T2 = 75
    BAT_1_MAJOR_ALERT_1 = 76
    BAT_1_MAJOR_ALERT_2 = 77
    BAT_1_MAJOR_ALERT_3 = 78
    BAT_1_MINOR_ALERT = 79
    BAT_2_AH_DISCHARGED = 80
    BAT_2_SOC = 81
    BAT_2_SOH = 82
    BAT_2_I_BAT_I = 83
    BAT_2_REMAINING_CAPACITY = 84
    BAT_2_T0 = 85
    BAT_2_T1 = 86
    BAT_2_T2 = 87
    BAT_2_MAJOR_ALERT_1 = 88
    BAT_2_MAJOR_ALERT_2 = 89
    BAT_2_MAJOR_ALERT_3 = 90
    BAT_2_MINOR_ALERT = 91
    BAT_3_AH_DISCHARGED = 92
    BAT_3_SOC = 93
    BAT_3_SOH = 94
    BAT_3_I_BAT_I = 95
    BAT_3_REMAINING_CAPACITY = 96
    BAT_3_T0 = 97
    BAT_3_T1 = 98
    BAT_3_T2 = 99
    BAT_3_MAJOR_ALERT_1 = 100
    BAT_3_MAJOR_ALERT_2 = 101
    BAT_3_MAJOR_ALERT_3 = 102
    BAT_3_MINOR_ALERT = 103


METRIC_TYPE_VALUES =  {value: name for name, value in vars(MetricTypes).items() if name.isupper()}