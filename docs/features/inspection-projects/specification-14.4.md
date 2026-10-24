# SolidWorks specification snapshot

Edition 0.128, 2026-10-24; approved ../inventor-interface working tree.

### 14.4 Inspection projects embedded in drawings

*Status: Partial · two native fixtures · Evidence: F.9*

Creating a project through the SOLIDWORKS Inspection add-in and saving the drawing adds `ThirdPty/InspectionXpert`. In the fixtures this stream is UTF-8 XML, starting with the UTF-8 BOM `EF BB BF` and an XML declaration, with unqualified root `ExtInspectionProject`. The root declares the `xsd` and `xsi` namespaces but the observed element paths have no namespace. The generated [Inspection XML schema inventory](inspection-schema.md) records 233 paths across the two fixtures, including option names, collection paths and observed lexical forms.

| Root child | Observed structure |
|---|---|
| `GUID`, `Name` | Text `SOLIDWORKS Inspection`; `GUID` is not necessarily a UUID. |
| `State` | Text `Selected`. Other states are not established. |
| `pObjID_Array` | Empty in these fixtures; nonempty structure open. |
| `ExtCADDrawing` | Drawing `GUID`, `Name`, `State`, `Path`, custom properties, referenced models, sheet data and flags. |
| `Settings` | Project text fields, sampling, options, project-template data and custom properties. |
| `Version`, `MajorVersion` | Add-in strings `2026.2.1.2` and `2026`; not binary archive-version selectors. |

`ExtCADDrawing/CustomProperties` can hold repeated `ExtProperty` elements. Observed children are `prop_type`, `Name`, `Value` and `PropertySource`, with `prop_type` = `text` and source `DrawingSummary`. The collection is empty in the edited fixture: a reader preserves this distinction. `ExtSheet` holds `GUID`, `Name`, `State` and `SheetIndex`; the fixture has Sheet1 at index 1. Nonempty referenced-model and characteristic structures remain open.

The General Settings control labelled Job # writes `Settings/JobNumber`: editing it from empty to `SCHEMA-JOB-001` stores that exact text. The other direct project text children are `Vendor`, `PartNumber`, `PartName`, `PartRevision`, `DocumentNumber`, `DocumentName` and `DocumentRevision`. Sampling includes `InspectionLevel`, `InspectionType`, `LotSize`, `AQL` and `InspectionMethod`. Options include extraction flags, units, balloon appearance and offsets, regex strings and export settings. Their complete observed paths are in the inventory; numeric units and full enum domains remain unverified (OI-019).

The XML `Path` and drawing `Name` retain the originating blank drawing in these Save As fixtures. They cannot be assumed to equal the current carrier's filesystem name. Empty collections, boolean-looking text, number-looking text and regex strings retain their XML spelling. Rewriting them by inferred type or updating cached paths without evidence is not specified here.
