use super::MessageType;
pub fn match_message_timestamp_field(mt: MessageType) -> Option<usize> {
    match mt {
        MessageType::AccelerometerData => Some(253),
        MessageType::Activity => Some(253),
        MessageType::AntRx => Some(253),
        MessageType::AntTx => Some(253),
        MessageType::AviationAttitude => Some(253),
        MessageType::BarometerData => Some(253),
        MessageType::BloodPressure => Some(253),
        MessageType::CameraEvent => Some(253),
        MessageType::CoursePoint => Some(1),
        MessageType::DeviceInfo => Some(253),
        MessageType::DiveSummary => Some(253),
        MessageType::Event => Some(253),
        MessageType::GpsMetadata => Some(253),
        MessageType::GyroscopeData => Some(253),
        MessageType::Hr => Some(253),
        MessageType::Jump => Some(253),
        MessageType::Lap => Some(253),
        MessageType::Length => Some(253),
        MessageType::MagnetometerData => Some(253),
        MessageType::Monitoring => Some(253),
        MessageType::MonitoringInfo => Some(253),
        MessageType::NmeaSentence => Some(253),
        MessageType::ObdiiData => Some(253),
        MessageType::OhrSettings => Some(253),
        MessageType::OneDSensorCalibration => Some(253),
        MessageType::Record => Some(253),
        MessageType::SegmentLap => Some(253),
        MessageType::Session => Some(253),
        MessageType::Set => Some(254),
        MessageType::ThreeDSensorCalibration => Some(253),
        MessageType::TimestampCorrelation => Some(253),
        MessageType::Totals => Some(253),
        MessageType::TrainingFile => Some(253),
        MessageType::VideoClip => Some(1),
        MessageType::VideoFrame => Some(253),
        MessageType::WeatherAlert => Some(253),
        MessageType::WeatherConditions => Some(253),
        MessageType::WeightScale => Some(253),
        _ => None
    }
}
