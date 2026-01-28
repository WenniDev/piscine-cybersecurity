macro_rules! define_tags {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident = $code:expr => $display:expr
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $(
                $(#[$vmeta])*
                $variant,
            )*
            Unknown,
        }

        impl From<u16> for $name {
            fn from(tag: u16) -> Self {
                match tag {
                    $(
                        $code => $name::$variant,
                    )*
                    _ => $name::Unknown,
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    $(
                        $name::$variant => $display,
                    )*
                    $name::Unknown => "Unknown",
                };
                write!(f, "{}", name)
            }
        }
    };
}

define_tags! {
    pub enum Tags {
        // TIFF tags
        ImageWidth = 0x100 => "Image Width",
        ImageLength = 0x101 => "Image Height",
        BitsPerSample = 0x102 => "Bits Per Sample",
        Compression = 0x103 => "Compression",
        PhotometricInterpretation = 0x106 => "Photometric Interpretation",
        Orientation = 0x112 => "Orientation",
        SamplesPerPixel = 0x115 => "Samples Per Pixel",
        PlanarConfiguration = 0x11C => "Planar Configuration",
        YCbCrSubSampling = 0x212 => "Y Cb Cr Sub Sampling",
        YCbCrPositioning = 0x213 => "Y Cb Cr Positioning",
        XResolution = 0x11A => "X Resolution",
        YResolution = 0x11B => "Y Resolution",
        ResolutionUnit = 0x128 => "Resolution Unit",
        StripOffsets = 0x111 => "Strip Offsets",
        RowsPerStrip = 0x116 => "Rows Per Strip",
        StripByteCounts = 0x117 => "Strip Byte Counts",
        JPEGInterchangeFormat = 0x201 => "JPEG Interchange Format",
        JPEGInterchangeFormatLength = 0x202 => "JPEG Interchange Format Length",
        TransferFunction = 0x12D => "Transfer Function",
        WhitePoint = 0x13E => "White Point",
        PrimaryChromaticities = 0x13F => "Primary Chromaticities",
        YCbCrCoefficients = 0x211 => "Y Cb Cr Coefficients",
        ReferenceBlackWhite = 0x214 => "Reference Black White",
        DateTime = 0x132 => "Modify Date",
        ImageDescription = 0x10E => "Image Description",
        Make = 0x10F => "Make",
        Model = 0x110 => "Camera Model Name",
        Software = 0x131 => "Software",
        Artist = 0x13B => "Artist",
        Copyright = 0x8298 => "Copyright",

        // IFD Pointers
        ExifIFDPointer = 0x8769 => "Exif IFD Pointer",
        GPSInfoIFDPointer = 0x8825 => "GPS Info IFD Pointer",
        InteroperabilityIFDPointer = 0xA005 => "Interoperability IFD Pointer",

        // EXIF tags
        ExposureTime = 0x829A => "Exposure Time",
        FNumber = 0x829D => "F Number",
        ISO = 0x8827 => "ISO",
        DateTimeOriginal = 0x9003 => "Date/Time Original",
        DateTimeDigitized = 0x9004 => "Create Date",
        ShutterSpeedValue = 0x9201 => "Shutter Speed Value",
        ApertureValue = 0x9202 => "Aperture Value",
        BrightnessValue = 0x9203 => "Brightness Value",
        ExposureBiasValue = 0x9204 => "Exposure Compensation",
        MaxApertureValue = 0x9205 => "Max Aperture Value",
        MeteringMode = 0x9207 => "Metering Mode",
        Flash = 0x9209 => "Flash",
        FocalLength = 0x920A => "Focal Length",
        SubjectArea = 0x9214 => "Subject Area",
        MakerNote = 0x927C => "Maker Note",
        UserComment = 0x9286 => "User Comment",
        SubSecTime = 0x9290 => "Sub Sec Time",
        SubSecTimeOriginal = 0x9291 => "Sub Sec Time Original",
        SubSecTimeDigitized = 0x9292 => "Sub Sec Time Digitized",
        FlashpixVersion = 0xA000 => "Flashpix Version",
        ColorSpace = 0xA001 => "Color Space",
        PixelXDimension = 0xA002 => "Exif Image Width",
        PixelYDimension = 0xA003 => "Exif Image Height",
        SensingMethod = 0xA217 => "Sensing Method",
        SceneType = 0xA301 => "Scene Type",
        ExposureMode = 0xA402 => "Exposure Mode",
        WhiteBalance = 0xA403 => "White Balance",
        DigitalZoomRatio = 0xA404 => "Digital Zoom Ratio",
        FocalLengthIn35mmFormat = 0xA405 => "Focal Length In 35mm Format",
        SceneCaptureType = 0xA406 => "Scene Capture Type",
        LensSpecification = 0xA432 => "Lens Info",
        LensMake = 0xA433 => "Lens Make",
        LensModel = 0xA434 => "Lens Model",

        // GPS tags
        GPSVersionID = 0x0000 => "GPS Version ID",
        GPSLatitudeRef = 0x0001 => "GPS Latitude Ref",
        GPSLatitude = 0x0002 => "GPS Latitude",
        GPSLongitudeRef = 0x0003 => "GPS Longitude Ref",
        GPSLongitude = 0x0004 => "GPS Longitude",
        GPSAltitudeRef = 0x0005 => "GPS Altitude Ref",
        GPSAltitude = 0x0006 => "GPS Altitude",
        GPSTimeStamp = 0x0007 => "GPS Time Stamp",
        GPSSatellites = 0x0008 => "GPS Satellites",
        GPSStatus = 0x0009 => "GPS Status",
        GPSMeasureMode = 0x000A => "GPS Measure Mode",
        GPSDateStamp = 0x001D => "GPS Date Stamp",
    }
}

impl Tags {
    pub fn is_sub_ifd_pointer(&self) -> bool {
        matches!(
            self,
            Tags::ExifIFDPointer | Tags::GPSInfoIFDPointer | Tags::InteroperabilityIFDPointer
        )
    }
}
