use ecow::EcoString;
use typst_syntax::Spanned;

use crate::World;
use crate::diag::At;
use crate::foundations::{Bytes, Cast, Derived, elem};
use crate::introspection::Locatable;

/// A file that will be embedded into the output PDF.
///
/// This can be used to distribute additional files that are related to the PDF
/// within it. PDF readers will display the files in a file listing.
///
/// Some international standards use this mechanism to embed machine-readable
/// data (e.g., ZUGFeRD/Factur-X for invoices) that mirrors the visual content
/// of the PDF.
///
/// # Example
/// ```typ
/// #pdf.embed(
///   "experiment.csv",
///   relationship: "supplement",
///   mime-type: "text/csv",
///   description: "Raw Oxygen readings from the Arctic experiment",
/// )
/// ```
///
/// # Notes
/// - This element is ignored if exporting to a format other than PDF.
/// - File embeddings are not currently supported for PDF/A-2, even if the
///   embedded file conforms to PDF/A-1 or PDF/A-2.
#[elem(Locatable)]
pub struct EmbedElem {
    /// The [path]($syntax/#paths) of the file to be embedded.
    ///
    /// Must always be specified, but is only read from if no data is provided
    /// in the following argument.
    #[required]
    #[parse(
        let Spanned { v: path, span } =
            args.expect::<Spanned<EcoString>>("path")?;
        let id = span.resolve_path(&path).at(span)?;
        // The derived part is the project-relative resolved path.
        let resolved = id.vpath().as_rootless_path().to_string_lossy().replace("\\", "/").into();
        Derived::new(path.clone(), resolved)
    )]
    pub path: Derived<EcoString, EcoString>,

    /// Raw file data, optionally.
    ///
    /// If omitted, the data is read from the specified path.
    #[positional]
    // Not actually required as an argument, but always present as a field.
    // We can't distinguish between the two at the moment.
    #[required]
    #[parse(
        match args.eat::<Bytes>()? {
            Some(data) => data,
            None => engine.world.file(id).at(span)?,
        }
    )]
    pub data: Bytes,

    /// The relationship of the embedded file to the document.
    ///
    /// Ignored if export doesn't target PDF/A-3.
    pub relationship: Option<EmbeddedFileRelationship>,

    /// The MIME type of the embedded file.
    pub mime_type: Option<EcoString>,

    /// A description for the embedded file.
    pub description: Option<EcoString>,
}

/// The relationship of an embedded file with the document.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Cast)]
pub enum EmbeddedFileRelationship {
    /// The PDF document was created from the source file.
    Source,
    /// The file was used to derive a visual presentation in the PDF.
    Data,
    /// An alternative representation of the document.
    Alternative,
    /// Additional resources for the document.
    Supplement,
}
