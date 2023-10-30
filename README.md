# type-lib
Defines all common types shared in the different WannSea components as individual libraries for the used languages.

## Custom Metric definitions
We use a custom metric definition file which defines all the metrics transmitted from the boat.
This allows us to generate unique IDs for every metric which then can be used during transmission to reduce package size.
In addition to that helper functions are generated for serializing/deserializing the metric values.
Everyone is free to add functionality to existing libraries or even add new libraries for non-implemented languages