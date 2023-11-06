|             name            | type |                                            description                                            |   unit  |
|-----------------------------|------|---------------------------------------------------------------------------------------------------|---------|
|           GPS_LON           |  f32 |                                           GPS Longitude                                           |    °    |
|           GPS_LAT           |  f32 |                                            GPS Latitude                                           |    °    |
|          GPS_SPEED          |  f32 |                                             GPS Speed                                             |    kn   |
|          GPS_COURSE         |  f32 |                                             GPS Course                                            |    °    |
|         GPS_ALTITUDE        |  f32 |                             GPS Altitude (above/below mean-sea-level)                             |    m    |
|        GPS_HOR_ERROR        |  f32 |                                        GPS Horizontal Error                                       |    m    |
|     GPS_VELO_UNCERTAINTY    |  f32 |                                      GPS Velocity Uncertainty                                     |   m/s   |
|     GPS_VERT_UNCERTAINTY    |  f32 |                                      GPS Vertical Uncertainty                                     |    m    |
|        ACCELERATION_X       |  f32 |                                       Acceleration in X-axis                                      |   m/s²  |
|        ACCELERATION_Y       |  f32 |                                       Acceleration in Y-axis                                      |   m/s²  |
|        ACCELERATION_Z       |  f32 |                                       Acceleration in Z-axis                                      |   m/s²  |
|            GYRO_X           |  f32 |                                           Gyro in X-axis                                          |    °    |
|            GYRO_Y           |  f32 |                                           Gyro in Y-axis                                          |    °    |
|            GYRO_Z           |  f32 |                                           Gyro in Z-axis                                          |    °    |
|          ROTATION_X         |  f32 |                                      Fused Rotation in X-axis                                     |    °    |
|          ROTATION_Y         |  f32 |                                      Fused Rotation in Y-axis                                     |    °    |
|          ROTATION_Z         |  f32 |                                      Fused Rotation in Z-axis                                     |    °    |
|          APMU_TEMP          |  f32 |                                          APMU Temperature                                         |   ° C   |
|          MPMU_TEMP          |  f32 |                                          MPMU Temperature                                         |   ° C   |
|        MOTOR_CURRENT        |  f32 |                                       Measured Motor Current                                      |    A    |
|       BATTERY_VOLTAGE       |  f32 |                                      Measured Battery Voltage                                     |    V    |
|            FAN_1            |  f32 |                                             Fan 1 RPM                                             |   RPM   |
|            FAN_2            |  f32 |                                             Fan 2 RPM                                             |   RPM   |
|            FAN_3            |  f32 |                                             Fan 3 RPM                                             |   RPM   |
|            FAN_4            |  f32 |                                             Fan 4 RPM                                             |   RPM   |
|         SOLAR_POWER         |  f32 |                                            Solar Power                                            |    W    |
|          SOLAR_TEMP         |  f32 |                                          MPPT Temperature                                         |   ° C   |
|MAX_BATTERY_DISCHARGE_CURRENT|  i16 |                                 Maximum Battery Discharge Current                                 |    A    |
| MAX_BATTERY_RECHARGE_CURRENT|  i16 |                                  Maximum Battery Recharge Current                                 |    A    |
|          GLOBAL_SOC         |  u8  |Global SOC of the system calculated from the SOC of all the batteries connected to the DC power bus|    %    |
|        ID_GLOBAL_SOC        |  u8  |                             ID of the pack which limits the global SOC                            |         |
|   GLOBAL_IBMS_ALARM_STATE   |  u8  |                        Global alert status: None (0), Minor (1), Major (2)                        |         |
|   NUMBER_OF_CONNECTED_BMS   |  u8  |                          Number of battery packs connected to the DC bus                          |         |
|     POWERBUS_INFORMATION    |  u8  |    Powerbus information: All child BMS connected (0), At least one child BMS not connected (1)    |         |
|           BAT_TMIN          |  f32 |                                                                                                   |         |
|           BAT_TMAX          |  f32 |                                                                                                   |         |
|         BAT_ID_TMIN         |  f32 |                                                                                                   |         |
|         BAT_ID_TMAX         |  f32 |                                                                                                   |         |
|           BAT_VMIN          |  f32 |                                                                                                   |         |
|           BAT_VMAX          |  f32 |                                                                                                   |         |
|         BAT_ID_VMIN         |  f32 |                                                                                                   |         |
|         BAT_ID_VMAX         |  f32 |                                                                                                   |         |
|      GLOBAL_BAT_CURRENT     |  f32 |                                                                                                   |         |
|       GLOBAL_CELL_MIN       |  f32 |                                                                                                   |         |
|       GLOBAL_CELL_MAX       |  f32 |                                                                                                   |         |
|      GLOBAL_CELL_MIN_ID     |  f32 |                                                                                                   |         |
|      GLOBAL_CELL_MAX_ID     |  f32 |                                                                                                   |         |
|          BAT_1_U_1          |  u16 |                       Battery 1: Cell 1 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_2          |  u16 |                       Battery 1: Cell 2 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_3          |  u16 |                       Battery 1: Cell 3 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_4          |  u16 |                       Battery 1: Cell 4 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_5          |  u16 |                       Battery 1: Cell 5 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_6          |  u16 |                       Battery 1: Cell 6 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_7          |  u16 |                       Battery 1: Cell 7 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_8          |  u16 |                       Battery 1: Cell 8 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_9          |  u16 |                       Battery 1: Cell 9 voltage measurement (on the module)                       |    mV   |
|          BAT_1_U_10         |  u16 |                       Battery 1: Cell 10 voltage measurement (on the module)                      |    mV   |
|          BAT_1_U_11         |  u16 |                       Battery 1: Cell 11 voltage measurement (on the module)                      |    mV   |
|          BAT_1_U_12         |  u16 |                       Battery 1: Cell 12 voltage measurement (on the module)                      |    mV   |
|          BAT_1_U_13         |  u16 |                       Battery 1: Cell 13 voltage measurement (on the module)                      |    mV   |
|          BAT_1_U_14         |  u16 |                       Battery 1: Cell 14 voltage measurement (on the module)                      |    mV   |
|     BAT_1_AH_DISCHARGED     |  u16 |                                 Battery 1: Discharged Ah counter.                                 |    Ah   |
|   BAT_1_REMAINING_CAPACITY  |  u16 |                               Battery 1: Remaining battery capacity.                              |    Ah   |
|          BAT_1_SOH          |  u8  |                                     Battery 1: State of Health                                    |    %    |
|          BAT_1_SOC          |  u8  |                                     Battery 1: State of Charge                                    |    %    |
|        BAT_1_I_BAT_I        |  u16 |               Battery 1: Absolute value of the current flowing through the battery.               |    A    |
|           BAT_1_T0          |  u8  |                            Battery 1: Sensor 1 temperature measurement                            |    °C   |
|           BAT_1_T1          |  u8  |                            Battery 1: Sensor 2 temperature measurement                            |    °C   |
|           BAT_1_T2          |  u8  |                            Battery 1: Sensor 3 temperature measurement                            |    °C   |
|     BAT_1_MAJOR_ALERT_1     |  u8  |                                     Battery 1: Major Alerts 1                                     |Unknown 😭|
|     BAT_1_MAJOR_ALERT_2     |  u8  |                                     Battery 1: Major Alerts 2                                     |Unknown 😭|
|     BAT_1_MAJOR_ALERT_3     |  u8  |                                     Battery 1: Major Alerts 3                                     |Unknown 😭|
|      BAT_1_MINOR_ALERT      |  u8  |                                      Battery 1: Minor Alerts                                      |Unknown 😭|
|          BAT_2_U_1          |  u16 |                       Battery 2: Cell 1 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_2          |  u16 |                       Battery 2: Cell 2 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_3          |  u16 |                       Battery 2: Cell 3 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_4          |  u16 |                       Battery 2: Cell 4 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_5          |  u16 |                       Battery 2: Cell 5 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_6          |  u16 |                       Battery 2: Cell 6 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_7          |  u16 |                       Battery 2: Cell 7 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_8          |  u16 |                       Battery 2: Cell 8 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_9          |  u16 |                       Battery 2: Cell 9 voltage measurement (on the module)                       |    mV   |
|          BAT_2_U_10         |  u16 |                       Battery 2: Cell 10 voltage measurement (on the module)                      |    mV   |
|          BAT_2_U_11         |  u16 |                       Battery 2: Cell 11 voltage measurement (on the module)                      |    mV   |
|          BAT_2_U_12         |  u16 |                       Battery 2: Cell 12 voltage measurement (on the module)                      |    mV   |
|          BAT_2_U_13         |  u16 |                       Battery 2: Cell 13 voltage measurement (on the module)                      |    mV   |
|          BAT_2_U_14         |  u16 |                       Battery 2: Cell 14 voltage measurement (on the module)                      |    mV   |
|     BAT_2_AH_DISCHARGED     |  u16 |                                 Battery 2: Discharged Ah counter.                                 |    Ah   |
|   BAT_2_REMAINING_CAPACITY  |  u16 |                               Battery 2: Remaining battery capacity.                              |    Ah   |
|          BAT_2_SOH          |  u8  |                                     Battery 2: State of Health                                    |    %    |
|          BAT_2_SOC          |  u8  |                                     Battery 2: State of Charge                                    |    %    |
|        BAT_2_I_BAT_I        |  u16 |               Battery 2: Absolute value of the current flowing through the battery.               |    A    |
|           BAT_2_T0          |  u8  |                            Battery 2: Sensor 1 temperature measurement                            |    °C   |
|           BAT_2_T1          |  u8  |                            Battery 2: Sensor 2 temperature measurement                            |    °C   |
|           BAT_2_T2          |  u8  |                            Battery 2: Sensor 3 temperature measurement                            |    °C   |
|     BAT_2_MAJOR_ALERT_1     |  u8  |                                     Battery 2: Major Alerts 1                                     |Unknown 😭|
|     BAT_2_MAJOR_ALERT_2     |  u8  |                                     Battery 2: Major Alerts 2                                     |Unknown 😭|
|     BAT_2_MAJOR_ALERT_3     |  u8  |                                     Battery 2: Major Alerts 3                                     |Unknown 😭|
|      BAT_2_MINOR_ALERT      |  u8  |                                      Battery 2: Minor Alerts                                      |Unknown 😭|
|          BAT_3_U_1          |  u16 |                       Battery 3: Cell 1 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_2          |  u16 |                       Battery 3: Cell 2 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_3          |  u16 |                       Battery 3: Cell 3 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_4          |  u16 |                       Battery 3: Cell 4 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_5          |  u16 |                       Battery 3: Cell 5 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_6          |  u16 |                       Battery 3: Cell 6 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_7          |  u16 |                       Battery 3: Cell 7 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_8          |  u16 |                       Battery 3: Cell 8 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_9          |  u16 |                       Battery 3: Cell 9 voltage measurement (on the module)                       |    mV   |
|          BAT_3_U_10         |  u16 |                       Battery 3: Cell 10 voltage measurement (on the module)                      |    mV   |
|          BAT_3_U_11         |  u16 |                       Battery 3: Cell 11 voltage measurement (on the module)                      |    mV   |
|          BAT_3_U_12         |  u16 |                       Battery 3: Cell 12 voltage measurement (on the module)                      |    mV   |
|          BAT_3_U_13         |  u16 |                       Battery 3: Cell 13 voltage measurement (on the module)                      |    mV   |
|          BAT_3_U_14         |  u16 |                       Battery 3: Cell 14 voltage measurement (on the module)                      |    mV   |
|     BAT_3_AH_DISCHARGED     |  u16 |                                 Battery 3: Discharged Ah counter.                                 |    Ah   |
|   BAT_3_REMAINING_CAPACITY  |  u16 |                               Battery 3: Remaining battery capacity.                              |    Ah   |
|          BAT_3_SOH          |  u8  |                                     Battery 3: State of Health                                    |    %    |
|          BAT_3_SOC          |  u8  |                                     Battery 3: State of Charge                                    |    %    |
|        BAT_3_I_BAT_I        |  u16 |               Battery 3: Absolute value of the current flowing through the battery.               |    A    |
|           BAT_3_T0          |  u8  |                            Battery 3: Sensor 1 temperature measurement                            |    °C   |
|           BAT_3_T1          |  u8  |                            Battery 3: Sensor 2 temperature measurement                            |    °C   |
|           BAT_3_T2          |  u8  |                            Battery 3: Sensor 3 temperature measurement                            |    °C   |
|     BAT_3_MAJOR_ALERT_1     |  u8  |                                     Battery 3: Major Alerts 1                                     |Unknown 😭|
|     BAT_3_MAJOR_ALERT_2     |  u8  |                                     Battery 3: Major Alerts 2                                     |Unknown 😭|
|     BAT_3_MAJOR_ALERT_3     |  u8  |                                     Battery 3: Major Alerts 3                                     |Unknown 😭|
|      BAT_3_MINOR_ALERT      |  u8  |                                      Battery 3: Minor Alerts                                      |Unknown 😭|
|         MEM_USED_MB         |  f32 |                                                                                                   |         |
|          MEM_TOTAL          |  f32 |                                                                                                   |         |
|         SWAP_USED_MB        |  f32 |                                                                                                   |         |
|          SWAP_TOTAL         |  f32 |                                                                                                   |         |
|        SYSTEM_UPTIME        |  f32 |                                                                                                   |         |
|        CPU_USAGE_USER       |  f32 |                                                                                                   |         |
|       CPU_USAGE_SYSTEM      |  f32 |                                                                                                   |         |
|           CPU_FREQ          |  f32 |                                                                                                   |         |
|          RAM_USAGE          |  f32 |                                                                                                   |         |
|           CPU_TEMP          |  f32 |                                                                                                   |         |
|        TX_IN_PER_SEC        |  f32 |                                                                                                   |         |
|        TX_OUT_PER_SEC       |  f32 |                                                                                                   |         |
|        TX_QUEUE_COUNT       |  f32 |                                                                                                   |         |
|   CELLULAR_SIGNAL_QUALITY   |  f32 |                                                                                                   |         |
|    CELLULAR_NETWORK_MODE    |String|                                                                                                   |         |