"""Owned XML lexical, namespace, encoding and section-scope fixtures."""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3] / "tests/fixtures/xml"
CUSTOM = "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties"
VT = "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes"
SOURCE = f'''<?xml version='1.0' encoding='UTF-8'?>\r
<!--before--><c:Properties xmlns:c="{CUSTOM}" xmlns="{CUSTOM}" xmlns:v="{VT}" future='kept'>
 <propertySection xmlns="" name = 'Custom' fmtid="section-one">
  <property name="" pid="20" TypeID="30" IsEquation="No">
   <v:lpstr xml:space='preserve'>A &amp; B<![CDATA[<MOD-DIAM>]]><!--note--><?keep x?>&#13;Z</v:lpstr>
   <FPVals><v:i4>-0002</v:i4><v:lpstr>fallback</v:lpstr></FPVals>
   <v:blob>AA==</v:blob><v:bool>No</v:bool><v:empty />
   <v:i2>+0001</v:i2><v:r8>1,5</v:r8><f:future xmlns:f="urn:future">unknown</f:future>
  </property>
  <propertyNameDictionaryElement name="" pid="20"/>
  <propertyNameDictionaryElement name="SW-Author" pid="20"/>
  <propertyNameDictionaryElement name="SW-Author(Author)" pid="20"/>
 </propertySection>
 <propertySection xmlns="" name="Other" fmtid="section-two"><property name="same-id" pid="20"><v:lpstr>second section</v:lpstr></property></propertySection>
</c:Properties>\r
'''


def write_encodings() -> None:
    (ROOT / "properties-utf8.bin").write_bytes(SOURCE.encode())
    (ROOT / "properties-utf8-bom.bin").write_bytes(b"\xef\xbb\xbf"+SOURCE.encode())
    for codec,bom,label in (("utf-16le",b"\xff\xfe","UTF-16"),("utf-16be",b"\xfe\xff","UTF-16"),
                            ("utf-16le",b"","UTF-16LE"),("utf-16be",b"","UTF-16BE")):
        text = SOURCE.replace("UTF-8",label)
        suffix = "-bom" if bom else ""
        (ROOT / f"properties-{codec}{suffix}.bin").write_bytes(bom+text.encode(codec))


def write_lexical_edges() -> None:
    (ROOT / "leaf.xml").write_text("<Tables q='a&gt;b'>A<![CDATA[B & <]]>C<!--note--><?keep x='y'?>D</Tables>",encoding="utf-8",newline="\n")
    (ROOT / "long-attribute.xml").write_text("<Tables xmlns:a='urn:attribute' a:"+"X"*65536+" "*300+"="+" "*300+"'A &amp; B'/>",encoding="utf-8",newline="\n")
    for prefix in (0x85,0x86):
        text = '<?xml version="1.0"?><Keywords><Dimension unit="unknown">&lt;MOD-DIAM&gt; λ</Dimension></Keywords>'
        (ROOT / f"keywords-{prefix:02x}.bin").write_bytes(bytes([prefix])+text.encode())
    text = '<?xml version="1.0"?>\r\r\n<m:materials xmlns:m="http://www.solidworks.com/sldmaterials"><material>鋼</material></m:materials>'
    (ROOT / "material-utf16.bin").write_bytes(b"\xff\xfe"+text.encode("utf-16le"))


FAMILIES = [
    ("[Content_Types].xml","http://schemas.openxmlformats.org/package/2006/content-types","Types"),
    ("_rels/.rels","http://schemas.openxmlformats.org/package/2006/relationships","Relationships"),
    ("docProps/core.xml","http://schemas.openxmlformats.org/package/2006/metadata/core-properties","coreProperties"),
    ("docProps/app.xml","http://schemas.openxmlformats.org/officeDocument/2006/extended-properties","Properties"),
    ("docProps/custom.xml",CUSTOM,"Properties"),
    ("docProps/Config-0-Properties.xml","http://www.solidworks.com/config-properties","ConfigProperties"),
    ("docProps/Config-001-Cutlist-Properties.xml",None,"Configuration"),
    ("docProps/ISolidWorksInformation.xml","http://schemas.openxmlformats.org/officeDocument/2006/SolidworksInformation-properties","Properties"),
    ("docProps/OpenTime.xml","http://schemas.openxmlformats.org/officeDocument/2006/SolidworksOpenTime","Properties"),
    ("swXmlContents/Features","http://www.solidworks.com/sw2003/schema","swSolidWorks"),
    ("swXmlContents/COMPINSTANCETREE","http://www.solidworks.com/sw2003/schema","swSolidWorks"),
    ("swXmlContents/xmlstream-2","http://www.solidworks.com/sw2003/schema","swSolidWorks"),
    ("swXmlContents/xmlstream-3","http://www.solidworks.com/sw2003/schema","swSolidWorks"),
    ("swXmlContents/KeyWords",None,"Keywords"),("swXmlContents/CustomProperties",None,"DisplayItems"),
    ("swXmlContents/MATERIALTREE","http://www.solidworks.com/sldmaterials","materials"),("swXmlContents/Tables",None,"Tables"),
]


def write_families() -> None:
    rows = []
    for index,(stream,namespace,local) in enumerate(FAMILIES):
        name = f"family-{index}.xml"
        text = f'<p:{local} xmlns:p="{namespace}" />' if namespace else f"<{local} />"
        (ROOT / name).write_text(text,encoding="utf-8",newline="\n")
        rows.append(f"{stream}\t{name}\n")
    (ROOT / "families.tsv").write_text("".join(rows),encoding="utf-8",newline="\n")


if __name__ == "__main__":
    ROOT.mkdir(parents=True,exist_ok=True)
    write_encodings()
    write_lexical_edges()
    write_families()
