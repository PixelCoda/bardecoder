//! Extract data from an image

use failure::Fail;

mod qr;

pub use self::qr::QRExtractor;

/// Extract data from a prepared image, given the location as determined by the [`Detect`] step
///
/// PREP type should be the type if the image returned from the [`Prepare`] implementation
/// LOC type should be the relevant enclosed type from the [`Location`] enum
/// DATA type must equal the input type of the matching [`Decode`] implementation
///
/// Pre-implemented Extracts provided by this library that are included in the default [`Decoder`]:
/// * [`QRExtractor`]
///
/// # Example
/// ```
/// # extern crate bardecoder;
/// # extern crate image;
/// # use bardecoder::util::qr::{QRLocation, QRData, QRError};
/// # use image::GrayImage;
/// use bardecoder::extract::Extract;
///
/// struct MyExtractor {}
///
/// impl Extract<GrayImage, QRLocation, QRData, QRError> for MyExtractor {
///     fn extract(&self, threshold: &GrayImage, loc: QRLocation) -> Result<QRData, QRError> {
///         // extract data here
/// #        Ok(QRData::new(vec![], 0))
///     }
/// }
/// ```
///
/// with the corresponding impl Decode being the Example [`here`]
///
/// [`Location`]: ../detect/enum.Location.html
/// [`Decode`]: ../decode/trait.Decode.html
/// [`Decoder`]: ../struct.Decoder.html
/// [`Detect`]: ../detect/trait.Detect.html
/// [`Prepare`]: ../prepare/trait.Prepare.html
/// [`here`]: ../decode/trait.Decode.html
pub trait Extract<PREP, LOC, DATA, ERROR>
where
    ERROR: Fail,
{
    /// Does the actual extracting
    fn extract(&self, threshold: &PREP, loc: LOC) -> Result<DATA, ERROR>;
}
