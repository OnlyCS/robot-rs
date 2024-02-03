pub const DOWNLOAD_LIST: [&'static str; 19] = [
    "hal/hal-cpp/{version}/hal-cpp-{version}-linuxathena",
    "hal/hal-cpp/{version}/hal-cpp-{version}-headers",
    "ni-libraries/visa/{version}/visa-{version}-linuxathena",
    "ni-libraries/visa/{version}/visa-{version}-headers",
    "ni-libraries/netcomm/{version}/netcomm-{version}-linuxathena",
    "ni-libraries/netcomm/{version}/netcomm-{version}-headers",
    "ni-libraries/chipobject/{version}/chipobject-{version}-linuxathena",
    "ni-libraries/chipobject/{version}/chipobject-{version}-headers",
    "ni-libraries/runtime/{version}/runtime-{version}-linuxathena",
    "wpiutil/wpiutil-cpp/{version}/wpiutil-cpp-{version}-linuxathena",
    "wpiutil/wpiutil-cpp/{version}/wpiutil-cpp-{version}-headers",
    "wpilibc/wpilibc-cpp/{version}/wpilibc-cpp-{version}-linuxathena",
    "wpilibc/wpilibc-cpp/{version}/wpilibc-cpp-{version}-headers",
    "wpimath/wpimath-cpp/{version}/wpimath-cpp-{version}-linuxathena",
    "wpimath/wpimath-cpp/{version}/wpimath-cpp-{version}-headers",
    "ntcore/ntcore-cpp/{version}/ntcore-cpp-{version}-linuxathena",
    "ntcore/ntcore-cpp/{version}/ntcore-cpp-{version}-headers",
    "cscore/cscore-cpp/{version}/cscore-cpp-{version}-linuxathena",
    "cscore/cscore-cpp/{version}/cscore-cpp-{version}-headers",
];

pub fn downloads<'a>(version: &str) -> Vec<String> {
    DOWNLOAD_LIST
        .iter()
        .map(|i| i.replace("{version}", version))
        .map(|url| format!("https://frcmaven.wpi.edu/artifactory/release/edu/wpi/first/{url}.zip"))
        .collect()
}
