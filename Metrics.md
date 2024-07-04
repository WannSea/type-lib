| id|             name            |                                                   description                                                  |     unit     |
|---|-----------------------------|----------------------------------------------------------------------------------------------------------------|--------------|
| 0 |           GPS_POS           |                                           GPS Position (x Lat, y Lon)                                          |       °      |
| 1 |     GPS_SATELLITE_COUNT     |                                               Count of satellites                                              |       #      |
| 2 |          GPS_SPEED          |                                                    GPS Speed                                                   |      kn      |
| 3 |          GPS_COURSE         |                                                   GPS Course                                                   |       °      |
| 4 |         GPS_ALTITUDE        |                                    GPS Altitude (above/below mean-sea-level)                                   |       m      |
| 5 |        GPS_HOR_ERROR        |                                              GPS Horizontal Error                                              |       m      |
| 6 |     GPS_VELO_UNCERTAINTY    |                                            GPS Velocity Uncertainty                                            |      m/s     |
| 7 |     GPS_VERT_UNCERTAINTY    |                                            GPS Vertical Uncertainty                                            |       m      |
| 8 |       IMU_ACCELERATION      |                                               Acceleration (Vec3)                                              |     m/s²     |
| 11|           IMU_GYRO          |                                                   Gyro (Vec3)                                                  |       °      |
| 14|         IMU_ROTATION        |                                              Fused Rotation (Vec3)                                             |       °      |
| 17|          APMU_TEMP          |                                                APMU Temperature                                                |      °C      |
| 18|          MPMU_TEMP          |                                                MPMU Temperature                                                |      °C      |
| 19|        MOTOR_CURRENT        |                                             Measured Motor Current                                             |       A      |
| 20|       BATTERY_VOLTAGE       |                                            Measured Battery Voltage                                            |       V      |
| 21|            FAN_1            |                                                    Fan 1 RPM                                                   |      RPM     |
| 22|            FAN_2            |                                                    Fan 2 RPM                                                   |      RPM     |
| 23|            FAN_3            |                                                    Fan 3 RPM                                                   |      RPM     |
| 24|            FAN_4            |                                                    Fan 4 RPM                                                   |      RPM     |
| 25|         SOLAR_POWER         |                                                   Solar Power                                                  |       W      |
| 26|          SOLAR_TEMP         |                                                MPPT Temperature                                                |      °C      |
| 27|MAX_BATTERY_DISCHARGE_CURRENT|                                        Maximum Battery Discharge Current                                       |       A      |
| 28| MAX_BATTERY_RECHARGE_CURRENT|                                        Maximum Battery Recharge Current                                        |       A      |
| 29|          GLOBAL_SOC         |       Global SOC of the system calculated from the SOC of all the batteries connected to the DC power bus      |       %      |
| 30|        ID_GLOBAL_SOC        |                                   ID of the pack which limits the global SOC                                   |              |
| 31|   GLOBAL_IBMS_ALARM_STATE   |                               Global alert status: None (0), Minor (1), Major (2)                              |              |
| 32|   NUMBER_OF_CONNECTED_BMS   |                                 Number of battery packs connected to the DC bus                                |              |
| 33|     POWERBUS_INFORMATION    |           Powerbus information: All child BMS connected (0), At least one child BMS not connected (1)          |              |
| 34|           BAT_TMIN          |                       Minimum temperature among all the BMS communicating on the CAN bus.                      |      °C      |
| 35|           BAT_TMAX          |                       Maximum temperature among all the BMS communicating on the CAN bus.                      |      °C      |
| 36|         BAT_ID_TMIN         |                                   ID of the BMS with the minimum temperature.                                  |       -      |
| 37|         BAT_ID_TMAX         |                                   ID of the BMS with the maximum temperature.                                  |       -      |
| 38|           BAT_VMIN          |                     Minimum battery voltage among all the BMS communicating on the CAN bus.                    |       V      |
| 39|           BAT_VMAX          |                     Maximum battery voltage among all the BMS communicating on the CAN bus.                    |       V      |
| 40|         BAT_ID_VMIN         |                                 ID of the BMS with the minimum battery voltage.                                |       -      |
| 41|         BAT_ID_VMAX         |                                 ID of the BMS with the maximum battery voltage.                                |       -      |
| 42|      GLOBAL_BAT_CURRENT     |                 Sum of the current flowing through all the modules connected to the power bus.                 |       A      |
| 43|      GLOBAL_CELL_V_MIN      |                    Minimum cell voltage among all the modules communicating on the CAN bus.                    |      mV      |
| 44|      GLOBAL_CELL_V_MAX      |                    Maximum cell voltage among all the modules communicating on the CAN bus.                    |      mV      |
| 45|     GLOBAL_CELL_V_MIN_ID    |                                ID of the battery with the minimum cell voltage.                                |       -      |
| 46|     GLOBAL_CELL_V_MAX_ID    |                                 ID of the battery with the maximum cell voltage                                |       -      |
| 47|          BAT_1_U_1          |                              Battery 1: Cell 1 voltage measurement (on the module)                             |      mV      |
| 48|          BAT_1_U_2          |                              Battery 1: Cell 2 voltage measurement (on the module)                             |      mV      |
| 49|          BAT_1_U_3          |                              Battery 1: Cell 3 voltage measurement (on the module)                             |      mV      |
| 50|          BAT_1_U_4          |                              Battery 1: Cell 4 voltage measurement (on the module)                             |      mV      |
| 51|          BAT_1_U_5          |                              Battery 1: Cell 5 voltage measurement (on the module)                             |      mV      |
| 52|          BAT_1_U_6          |                              Battery 1: Cell 6 voltage measurement (on the module)                             |      mV      |
| 53|          BAT_1_U_7          |                              Battery 1: Cell 7 voltage measurement (on the module)                             |      mV      |
| 54|          BAT_1_U_8          |                              Battery 1: Cell 8 voltage measurement (on the module)                             |      mV      |
| 55|          BAT_1_U_9          |                              Battery 1: Cell 9 voltage measurement (on the module)                             |      mV      |
| 56|          BAT_1_U_10         |                             Battery 1: Cell 10 voltage measurement (on the module)                             |      mV      |
| 57|          BAT_1_U_11         |                             Battery 1: Cell 11 voltage measurement (on the module)                             |      mV      |
| 58|          BAT_1_U_12         |                             Battery 1: Cell 12 voltage measurement (on the module)                             |      mV      |
| 59|          BAT_1_U_13         |                             Battery 1: Cell 13 voltage measurement (on the module)                             |      mV      |
| 60|          BAT_1_U_14         |                             Battery 1: Cell 14 voltage measurement (on the module)                             |      mV      |
| 61|     BAT_1_AH_DISCHARGED     |                                        Battery 1: Discharged Ah counter.                                       |      Ah      |
| 62|   BAT_1_REMAINING_CAPACITY  |                                     Battery 1: Remaining battery capacity.                                     |      Ah      |
| 63|          BAT_1_SOH          |                                           Battery 1: State of Health                                           |       %      |
| 64|          BAT_1_SOC          |                                           Battery 1: State of Charge                                           |       %      |
| 65|        BAT_1_I_BAT_I        |                      Battery 1: Absolute value of the current flowing through the battery.                     |       A      |
| 66|           BAT_1_T0          |                                   Battery 1: Sensor 1 temperature measurement                                  |      °C      |
| 67|           BAT_1_T1          |                                   Battery 1: Sensor 2 temperature measurement                                  |      °C      |
| 68|           BAT_1_T2          |                                   Battery 1: Sensor 3 temperature measurement                                  |      °C      |
| 69|     BAT_1_MAJOR_ALERT_1     |                                            Battery 1: Major Alerts 1                                           |  Unknown 😭  |
| 70|     BAT_1_MAJOR_ALERT_2     |                                            Battery 1: Major Alerts 2                                           |  Unknown 😭  |
| 71|     BAT_1_MAJOR_ALERT_3     |                                            Battery 1: Major Alerts 3                                           |  Unknown 😭  |
| 72|      BAT_1_MINOR_ALERT      |                                             Battery 1: Minor Alerts                                            |  Unknown 😭  |
| 73|          BAT_2_U_1          |                              Battery 2: Cell 1 voltage measurement (on the module)                             |      mV      |
| 74|          BAT_2_U_2          |                              Battery 2: Cell 2 voltage measurement (on the module)                             |      mV      |
| 75|          BAT_2_U_3          |                              Battery 2: Cell 3 voltage measurement (on the module)                             |      mV      |
| 76|          BAT_2_U_4          |                              Battery 2: Cell 4 voltage measurement (on the module)                             |      mV      |
| 77|          BAT_2_U_5          |                              Battery 2: Cell 5 voltage measurement (on the module)                             |      mV      |
| 78|          BAT_2_U_6          |                              Battery 2: Cell 6 voltage measurement (on the module)                             |      mV      |
| 79|          BAT_2_U_7          |                              Battery 2: Cell 7 voltage measurement (on the module)                             |      mV      |
| 80|          BAT_2_U_8          |                              Battery 2: Cell 8 voltage measurement (on the module)                             |      mV      |
| 81|          BAT_2_U_9          |                              Battery 2: Cell 9 voltage measurement (on the module)                             |      mV      |
| 82|          BAT_2_U_10         |                             Battery 2: Cell 10 voltage measurement (on the module)                             |      mV      |
| 83|          BAT_2_U_11         |                             Battery 2: Cell 11 voltage measurement (on the module)                             |      mV      |
| 84|          BAT_2_U_12         |                             Battery 2: Cell 12 voltage measurement (on the module)                             |      mV      |
| 85|          BAT_2_U_13         |                             Battery 2: Cell 13 voltage measurement (on the module)                             |      mV      |
| 86|          BAT_2_U_14         |                             Battery 2: Cell 14 voltage measurement (on the module)                             |      mV      |
| 87|     BAT_2_AH_DISCHARGED     |                                        Battery 2: Discharged Ah counter.                                       |      Ah      |
| 88|   BAT_2_REMAINING_CAPACITY  |                                     Battery 2: Remaining battery capacity.                                     |      Ah      |
| 89|          BAT_2_SOH          |                                           Battery 2: State of Health                                           |       %      |
| 90|          BAT_2_SOC          |                                           Battery 2: State of Charge                                           |       %      |
| 91|        BAT_2_I_BAT_I        |                      Battery 2: Absolute value of the current flowing through the battery.                     |       A      |
| 92|           BAT_2_T0          |                                   Battery 2: Sensor 1 temperature measurement                                  |      °C      |
| 93|           BAT_2_T1          |                                   Battery 2: Sensor 2 temperature measurement                                  |      °C      |
| 94|           BAT_2_T2          |                                   Battery 2: Sensor 3 temperature measurement                                  |      °C      |
| 95|     BAT_2_MAJOR_ALERT_1     |                                            Battery 2: Major Alerts 1                                           |  Unknown 😭  |
| 96|     BAT_2_MAJOR_ALERT_2     |                                            Battery 2: Major Alerts 2                                           |  Unknown 😭  |
| 97|     BAT_2_MAJOR_ALERT_3     |                                            Battery 2: Major Alerts 3                                           |  Unknown 😭  |
| 98|      BAT_2_MINOR_ALERT      |                                             Battery 2: Minor Alerts                                            |  Unknown 😭  |
| 99|          BAT_3_U_1          |                              Battery 3: Cell 1 voltage measurement (on the module)                             |      mV      |
|100|          BAT_3_U_2          |                              Battery 3: Cell 2 voltage measurement (on the module)                             |      mV      |
|101|          BAT_3_U_3          |                              Battery 3: Cell 3 voltage measurement (on the module)                             |      mV      |
|102|          BAT_3_U_4          |                              Battery 3: Cell 4 voltage measurement (on the module)                             |      mV      |
|103|          BAT_3_U_5          |                              Battery 3: Cell 5 voltage measurement (on the module)                             |      mV      |
|104|          BAT_3_U_6          |                              Battery 3: Cell 6 voltage measurement (on the module)                             |      mV      |
|105|          BAT_3_U_7          |                              Battery 3: Cell 7 voltage measurement (on the module)                             |      mV      |
|106|          BAT_3_U_8          |                              Battery 3: Cell 8 voltage measurement (on the module)                             |      mV      |
|107|          BAT_3_U_9          |                              Battery 3: Cell 9 voltage measurement (on the module)                             |      mV      |
|108|          BAT_3_U_10         |                             Battery 3: Cell 10 voltage measurement (on the module)                             |      mV      |
|109|          BAT_3_U_11         |                             Battery 3: Cell 11 voltage measurement (on the module)                             |      mV      |
|110|          BAT_3_U_12         |                             Battery 3: Cell 12 voltage measurement (on the module)                             |      mV      |
|111|          BAT_3_U_13         |                             Battery 3: Cell 13 voltage measurement (on the module)                             |      mV      |
|112|          BAT_3_U_14         |                             Battery 3: Cell 14 voltage measurement (on the module)                             |      mV      |
|113|     BAT_3_AH_DISCHARGED     |                                        Battery 3: Discharged Ah counter.                                       |      Ah      |
|114|   BAT_3_REMAINING_CAPACITY  |                                     Battery 3: Remaining battery capacity.                                     |      Ah      |
|115|          BAT_3_SOH          |                                           Battery 3: State of Health                                           |       %      |
|116|          BAT_3_SOC          |                                           Battery 3: State of Charge                                           |       %      |
|117|        BAT_3_I_BAT_I        |                      Battery 3: Absolute value of the current flowing through the battery.                     |       A      |
|118|           BAT_3_T0          |                                   Battery 3: Sensor 1 temperature measurement                                  |      °C      |
|119|           BAT_3_T1          |                                   Battery 3: Sensor 2 temperature measurement                                  |      °C      |
|120|           BAT_3_T2          |                                   Battery 3: Sensor 3 temperature measurement                                  |      °C      |
|121|     BAT_3_MAJOR_ALERT_1     |                                            Battery 3: Major Alerts 1                                           |  Unknown 😭  |
|122|     BAT_3_MAJOR_ALERT_2     |                                            Battery 3: Major Alerts 2                                           |  Unknown 😭  |
|123|     BAT_3_MAJOR_ALERT_3     |                                            Battery 3: Major Alerts 3                                           |  Unknown 😭  |
|124|      BAT_3_MINOR_ALERT      |                                             Battery 3: Minor Alerts                                            |  Unknown 😭  |
|133|        TX_IN_PER_SEC        |                                # of Metrics currently being collected per second                               |   Metrics/s  |
|134|        TX_OUT_PER_SEC       |                               # of Metrics currently being transmitted per second                              |   Metrics/s  |
|135|        TX_QUEUE_COUNT       |                            # of Metrics currently in queue waiting for transmission                            |       -      |
|136|   CELLULAR_SIGNAL_QUALITY   |AT Command Signal quality (refer to: https://m2msupport.net/m2msupport/atcsq-signal-quality/) for interpretation|       -      |
|137|    CELLULAR_NETWORK_MODE    |                                              Cellular Network Mode                                             |              |
|138|         NET_RX_BYTES        |                                             Network bytes received                                             |              |
|139|         NET_TX_BYTES        |                                            Network bytes transmitted                                           |              |
|140|        NET_RX_PACKETS       |                                            Network packets received                                            |              |
|141|        NET_TX_PACKETS       |                                           Network packets transmitted                                          |              |
|142|        NET_RX_ERORRS        |                                             Network erorrs received                                            |              |
|143|        NET_TX_ERORRS        |                                           Network errors transmitted                                           |              |
|144|   FUSED_POSITION_RELATIVE   |                               (x / y) Fused Position relative to reference Point                               |       m      |
|145|        FUSED_POSITION       |                                        (Lat / Lon) after IMU+GNSS Fusion                                       |       °      |
|146|  FUSED_POSITION_UNCERTAINTY |                                                 IMU+GNSS Fusion                                                |      tbd     |
|147|        FUSED_VELOCITY       |                                                 IMU+GNSS Fusion                                                |      tbd     |
|148|  FUSED_VELOCITY_UNCERTAINTY |                                                 IMU+GNSS Fusion                                                |      tbd     |
|149|      FUSED_ORIENTATION      |                                                 IMU+GNSS Fusion                                                |      tbd     |
|150|FUSED_ORIENTATION_UNCERTAINTY|                                                 IMU+GNSS Fusion                                                |      tbd     |
|151|      MOTOR_TEMPERATURE      |                                Motor Temperature measured by internal thermistor                               |      °C      |
|200|          CPU_FREQS          |                                                 CPU Frequencies                                                |      MHz     |
|201|           MEM_USED          |                                              System memory in use                                              |     Bytes    |
|202|          MEM_TOTAL          |                                          Total System memory available                                         |     Bytes    |
|203|          SWAP_USED          |                                               Swap memory in use                                               |     Bytes    |
|204|          SWAP_TOTAL         |                                           Total Swap memory available                                          |     Bytes    |
|205|        SYSTEM_UPTIME        |                                                  System uptime                                                 |       s      |
|206|        CPU_USAGE_USER       |                                                 User CPU usage                                                 |       %      |
|207|       CPU_USAGE_SYSTEM      |                                                System CPU usage                                                |       %      |
|208|           CPU_TEMP          |                                                 CPU Temperature                                                |      °C      |
|209|        PROC_CPU_USAGE       |                                              CPU Usage by process                                              |       %      |
|210|        PROC_MEM_USED        |                                             Memory Usage by process                                            |MegaBytes (MB)|
|300|           ESC_RPM           |                                              Rotations per Minute                                              |    min^-1    |
|301|      ESC_TOTAL_CURRENT      |                                         ESC Total Current by all Units                                         |       A      |
|302|        ESC_DUTY_CYCLE       |                                              Latest PWM Duty Cycle                                             |       %      |
|303|        ESC_AMP_HOURS        |                                            ESC consumed Ampere Hours                                           |      Ah      |
|304|    ESC_AMP_HOURS_CHARGED    |                                        ESC consumed Ampere Hours charged                                       |      Ah      |
|305|        ESC_WATT_HOURS       |                                             ESC consumed Watt Hours                                            |      Wh      |
|306|    ESC_WATT_HOURS_CHARGED   |                                         ESC consumed Watt Hours charged                                        |      Wh      |
|307|       ESC_MOSFET_TEMP       |                                             ESC MOSFET Temperature                                             |      °C      |
|308|        ESC_MOTOR_TEMP       |                                                Motor Temperature                                               |      °C      |
|309|     ESC_TOTAL_IN_CURRENT    |                                             ESC Total Input Current                                            |       A      |
|310|         ESC_PID_POS         |                                                ESC PID Position                                                |       #      |
|311|        ESC_TACHOMETER       |                              ESC Tachometer, assumed RPM? (interpretation unsure)                              |   min^-1 ?   |
|312|        ESC_IN_VOLTAGE       |                                               ESC Output Voltage                                               |       V      |
|313|         ESC_SET_DUTY        |                                           Commanded Duty Cycle on ESC                                          |       %      |
|314|       ESC_SET_CURRENT       |                                         Commanded current output on ESC                                        |      mA      |
|316|         ESC_SET_RPM         |                                              Commanded RPM on ESC                                              |    min^-1    |
|317|     ESC_SET_CURRENT_REL     |       Commanded relative current on ESC (If upper and lower limits are  asymmetric command 0 is not 0 A)       |       %      |
|318|     CELLULAR_MODULE_TEMP    |                                               Temp of LTE module                                               |      °C      |
|319|   CELLULAR_MODULE_VOLTAGE   |                                              Voltage of LTE module                                             |       V      |
|320|   CELLULAR_MODULE_LOG_MSG   |                                        Other less important log messages                                       |     Text     |
|321|         THROTTLE_POS        |                                                Throttle position                                               |       %      |
|323|           PCS_TEMP          |                                                    PCS Temp                                                    |      °C      |
|324|         LPMainPower         |                                                  LP Main Power                                                 |      mW      |