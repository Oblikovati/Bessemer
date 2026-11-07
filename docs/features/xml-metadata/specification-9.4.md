# SolidWorks specification snapshot

Edition 0.127, 2026-10-24. Copied from the approved ../inventor-interface working tree; its uncommitted documentation is evolving.

## 9.4 XML metadata and document summaries

*Status: Typed · observed structure in 1,115 container samples · Evidence: F.7*

The XML streams are stored as ordinary stream payloads after container decompression. They have no archive object header. The generated [XML structure inventory](xml-metadata.md) records 17 normalized stream families and 219 element paths, using expanded XML names and attribute occurrence counts. This is an observed schema, not an exhaustive XSD: counts do not establish required fields, ordering constraints or permitted values outside the sample set.

#### 9.4.1 Package metadata

*Status: Typed · observed XML structure · Evidence: F.7*

`[Content_Types].xml` has root `Types` in `http://schemas.openxmlformats.org/package/2006/content-types`. `Default` children associate `Extension` with `ContentType`; `Override` children associate `PartName` with `ContentType`. `_rels/.rels` has root `Relationships` in `http://schemas.openxmlformats.org/package/2006/relationships`; its `Relationship` children carry `Id`, `Type` and `Target`. These XML streams do not change the container signatures or stream-name encoding of clause 7.

`docProps/core.xml` has root `coreProperties` in `http://schemas.openxmlformats.org/package/2006/metadata/core-properties`. Readers SHOULD identify children by expanded name rather than prefix. For example, bolt.sldprt stores `lastModifiedBy` in the Dublin Core elements namespace (`http://purl.org/dc/elements/1.1/`), despite the familiar core-property spelling. Creation and modification fields use the Dublin Core terms namespace (`http://purl.org/dc/terms/`).

`docProps/app.xml` has root `Properties` in `http://schemas.openxmlformats.org/officeDocument/2006/extended-properties`. Its children include `Application`, `AppVersion`, `Company`, `Template`, `TotalTime` and document-status fields. Stored version text and date strings MUST NOT be used to select the binary archive version; use Annex B instead.

#### 9.4.2 Property sections

*Status: Typed · observed XML structure · Evidence: F.7*

The roots of `docProps/custom.xml`, `docProps/Config-<n>-Properties.xml` and `docProps/ISolidWorksInformation.xml` use different namespaces (listed in the inventory). Their `propertySection` children are unqualified: the files explicitly reset the default namespace with `xmlns=""`. A reader that searches for these children in the root namespace will miss the properties.

Each `propertySection` carries `name` and `fmtid`. Its `property` children carry `name` and `pid`, with typed value children in `http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes`. Observed value elements include `i2`, `i4`, `r8`, `lpstr`, `bool`, `empty` and `blob`; unqualified `FPVals` children also occur and contain typed values. Preserve these additional values rather than reducing every property to its first text node. The meaning of `FPVals` and the blob encoding remain open (OI-017).

`propertyNameDictionaryElement` children associate names with property IDs. Multiple names can refer to the same ID: bolt.sldprt has both `SW-Author` and `SW-Author(Author)` for pid 20. IDs are therefore interpreted within their section; they are not unique document-wide keys. Preserve empty names and dictionary entries. Observed `vt:bool` text includes `No`; do not assume every value follows the lexical conventions of XML Schema booleans. Property values retain their stored type and text until their interpretation is established.

The `ISolidWorksInformation` properties include saved file and configuration names, author information and unit settings. Enum mappings for the integer unit settings are not established here. `Config-<n>-Cutlist-Properties.xml` uses a separate, unqualified `Configuration` root; it MUST NOT be parsed as the `ConfigProperties` form. `docProps/OpenTime.xml` likewise has its own namespace and structure, listed in the inventory.

#### 9.4.3 Feature and configuration summaries

*Status: Typed · observed XML structure · Evidence: F.7*

`swXmlContents/Features` (919 parts) and `swXmlContents/COMPINSTANCETREE` (176 assemblies) have a `swSolidWorks` root in `http://www.solidworks.com/sw2003/schema`. The inventory lists their header, model, configuration and instance paths. These summaries contain names, IDs and references; they do not specify the feature definitions needed to rebuild a model. Their `swVersion` attribute is separate from the container version and the archive versions of Annex B. The rarer `xmlstream-2` and `xmlstream-3` streams use the same root namespace but have separately inventoried paths.

`swXmlContents/KeyWords` has one byte before its XML declaration: `85` in 170 streams, `86` in 945. A reader MUST remove that framing byte before XML parsing when the declaration starts at byte 1. Its meaning remains open (OI-005); it is not an XML encoding marker. The unqualified root is `Keywords`; children include configurations, features, sketches and dimensions. Dimension text can contain display markup such as `<MOD-DIAM>` after XML entity decoding, so it is not necessarily a floating-point value or an SI measurement.

`MATERIALTREE`, `CustomProperties` and `Tables` have distinct roots and can be empty streams. An empty payload is distinct from an XML document with an empty root. Their observed structure is recorded in the inventory; material constants, table value semantics and the completeness of the summaries remain open (OI-017).
