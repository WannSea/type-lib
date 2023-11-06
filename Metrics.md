|             name            | type |               description               |unit|
|-----------------------------|------|-----------------------------------------|----|
|           GPS_LON           |  f32 |              GPS Longitude              |  ° |
|           GPS_LAT           |  f32 |               GPS Latitude              |  ° |
|          GPS_SPEED          |  f32 |                GPS Speed                | kn |
|          GPS_COURSE         |  f32 |                GPS Course               |  ° |
|         GPS_ALTITUDE        |  f32 |GPS Altitude (above/below mean-sea-level)|  m |
|        GPS_HOR_ERROR        |  f32 |           GPS Horizontal Error          |  m |
|     GPS_VELO_UNCERTAINTY    |  f32 |         GPS Velocity Uncertainty        | m/s|
|     GPS_VERT_UNCERTAINTY    |  f32 |         GPS Vertical Uncertainty        |  m |
|        ACCELERATION_X       |  f32 |          Acceleration in X-axis         |m/s²|
|        ACCELERATION_Y       |  f32 |          Acceleration in Y-axis         |m/s²|
|        ACCELERATION_Z       |  f32 |          Acceleration in Z-axis         |m/s²|
|            GYRO_X           |  f32 |              Gyro in X-axis             |  ° |
|            GYRO_Y           |  f32 |              Gyro in Y-axis             |  ° |
|            GYRO_Z           |  f32 |              Gyro in Z-axis             |  ° |
|          ROTATION_X         |  f32 |         Fused Rotation in X-axis        |  ° |
|          ROTATION_Y         |  f32 |         Fused Rotation in Y-axis        |  ° |
|          ROTATION_Z         |  f32 |         Fused Rotation in Z-axis        |  ° |
|          APMU_TEMP          |  f32 |             APMU Temperature            | ° C|
|          MPMU_TEMP          |  f32 |             MPMU Temperature            | ° C|
|        MOTOR_CURRENT        |  f32 |          Measured Motor Current         |  A |
|       BATTERY_VOLTAGE       |  f32 |         Measured Battery Voltage        |  V |
|            FAN_1            |  f32 |                Fan 1 RPM                | RPM|
|            FAN_2            |  f32 |                Fan 2 RPM                | RPM|
|            FAN_3            |  f32 |                Fan 3 RPM                | RPM|
|            FAN_4            |  f32 |                Fan 4 RPM                | RPM|
|         SOLAR_POWER         |  f32 |               Solar Power               |  W |
|          SOLAR_TEMP         |  f32 |             MPPT Temperature            | ° C|
|MAX_BATTERY_DISCHARGE_CURRENT|  f32 |    Maximum Battery Discharge Current    |  A |
| MAX_BATTERY_RECHARGE_CURRENT|  f32 |     Maximum Battery Recharge Current    |  A |
|          GLOBAL_SOC         |  f32 |                                         |    |
|        ID_GLOBAL_SOC        |  f32 |                                         |    |
|   GLOBAL_IBMS_ALARM_STATE   |  f32 |                                         |    |
|   NUMBER_OF_CONNECTED_BMS   |  f32 |                                         |    |
|     POWERBUS_INFORMATION    |  f32 |                                         |    |
|           BAT_TMIN          |  f32 |                                         |    |
|           BAT_TMAX          |  f32 |                                         |    |
|         BAT_ID_TMIN         |  f32 |                                         |    |
|         BAT_ID_TMAX         |  f32 |                                         |    |
|           BAT_VMIN          |  f32 |                                         |    |
|           BAT_VMAX          |  f32 |                                         |    |
|         BAT_ID_VMIN         |  f32 |                                         |    |
|         BAT_ID_VMAX         |  f32 |                                         |    |
|      GLOBAL_BAT_CURRENT     |  f32 |                                         |    |
|       GLOBAL_CELL_MIN       |  f32 |                                         |    |
|       GLOBAL_CELL_MAX       |  f32 |                                         |    |
|      GLOBAL_CELL_MIN_ID     |  f32 |                                         |    |
|      GLOBAL_CELL_MAX_ID     |  f32 |                                         |    |
|     BAT_1_AH_DISCHARGED     |  f32 |                                         |    |
|          BAT_1_SOC          |  f32 |                                         |    |
|          BAT_1_SOH          |  f32 |                                         |    |
|        BAT_1_I_BAT_I        |  f32 |                                         |    |
|   BAT_1_REMAINING_CAPACITY  |  f32 |                                         |    |
|           BAT_1_T0          |  f32 |                                         |    |
|           BAT_1_T1          |  f32 |                                         |    |
|           BAT_1_T2          |  f32 |                                         |    |
|     BAT_1_MAJOR_ALERT_1     |  f32 |                                         |    |
|     BAT_1_MAJOR_ALERT_2     |  f32 |                                         |    |
|     BAT_1_MAJOR_ALERT_3     |  f32 |                                         |    |
|      BAT_1_MINOR_ALERT      |  f32 |                                         |    |
|     BAT_2_AH_DISCHARGED     |  f32 |                                         |    |
|          BAT_2_SOC          |  f32 |                                         |    |
|          BAT_2_SOH          |  f32 |                                         |    |
|        BAT_2_I_BAT_I        |  f32 |                                         |    |
|   BAT_2_REMAINING_CAPACITY  |  f32 |                                         |    |
|           BAT_2_T0          |  f32 |                                         |    |
|           BAT_2_T1          |  f32 |                                         |    |
|           BAT_2_T2          |  f32 |                                         |    |
|     BAT_2_MAJOR_ALERT_1     |  f32 |                                         |    |
|     BAT_2_MAJOR_ALERT_2     |  f32 |                                         |    |
|     BAT_2_MAJOR_ALERT_3     |  f32 |                                         |    |
|      BAT_2_MINOR_ALERT      |  f32 |                                         |    |
|     BAT_3_AH_DISCHARGED     |  f32 |                                         |    |
|          BAT_3_SOC          |  f32 |                                         |    |
|          BAT_3_SOH          |  f32 |                                         |    |
|        BAT_3_I_BAT_I        |  f32 |                                         |    |
|   BAT_3_REMAINING_CAPACITY  |  f32 |                                         |    |
|           BAT_3_T0          |  f32 |                                         |    |
|           BAT_3_T1          |  f32 |                                         |    |
|           BAT_3_T2          |  f32 |                                         |    |
|     BAT_3_MAJOR_ALERT_1     |  f32 |                                         |    |
|     BAT_3_MAJOR_ALERT_2     |  f32 |                                         |    |
|     BAT_3_MAJOR_ALERT_3     |  f32 |                                         |    |
|      BAT_3_MINOR_ALERT      |  f32 |                                         |    |
|         MEM_USED_MB         |  f32 |                                         |    |
|          MEM_TOTAL          |  f32 |                                         |    |
|         SWAP_USED_MB        |  f32 |                                         |    |
|          SWAP_TOTAL         |  f32 |                                         |    |
|        SYSTEM_UPTIME        |  f32 |                                         |    |
|        CPU_USAGE_USER       |  f32 |                                         |    |
|       CPU_USAGE_SYSTEM      |  f32 |                                         |    |
|           CPU_FREQ          |  f32 |                                         |    |
|          RAM_USAGE          |  f32 |                                         |    |
|           CPU_TEMP          |  f32 |                                         |    |
|        TX_IN_PER_SEC        |  f32 |                                         |    |
|        TX_OUT_PER_SEC       |  f32 |                                         |    |
|        TX_QUEUE_COUNT       |  f32 |                                         |    |
|   CELLULAR_SIGNAL_QUALITY   |  f32 |                                         |    |
|    CELLULAR_NETWORK_MODE    |String|                                         |    |