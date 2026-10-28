# ZIP File Format Specification — part 13

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-012.md)

<a id="source-page-1-part-13"></a>

## Source page 1 (continued, part 13)

```text
   0092     DSNTYPE Flags                       1 byte   
   0093     Total Space Allocation (Cyls)       4 bytes  Big Endian
   009D     NONVSAM DSORG                       2 bytes  
   009E     Program Virtual Object Info         3 bytes  
   009F     Encapsulated file Info              9 bytes
   00A2     Cluster Log                         4 bytes  Binary
   00A3     Cluster LSID Length                 4 bytes  Binary
   00A4     Cluster LSID                       26 bytes  EBCDIC
   400C     Unix File Creation Time             4 bytes
   400D     Unix File Access Time               4 bytes
   400E     Unix File Modification time         4 bytes
   4101     IBMCMPSC Compression Info           variable
   4102     IBMCMPSC Compression Size           8 bytes  Big Endian

APPENDIX C - Zip64 Extensible Data Sector Mappings 
---------------------------------------------------

         -Z390   Extra Field:

          The following is the general layout of the attributes for the 
          ZIP 64 "extra" block for extended tape operations.  

          Note: some fields stored in Big Endian format.  All text is 
          in EBCDIC format unless otherwise specified.

          Value       Size          Description
          -----       ----          -----------
  (Z390)  0x0065      2 bytes       Tag for this "extra" block type
          Size        4 bytes       Size for the following data block
          Tag         4 bytes       EBCDIC "Z390"
          Length71    2 bytes       Big Endian
          Subcode71   2 bytes       Enote type code
          FMEPos      1 byte
          Length72    2 bytes       Big Endian
          Subcode72   2 bytes       Unit type code
          Unit        1 byte        Unit
          Length73    2 bytes       Big Endian
          Subcode73   2 bytes       Volume1 type code
          FirstVol    1 byte        Volume
          Length74    2 bytes       Big Endian
          Subcode74   2 bytes       FirstVol file sequence
          FileSeq     2 bytes       Sequence 

APPENDIX D - Language Encoding (EFS)
------------------------------------

D.1 The ZIP format has historically supported only the original IBM PC character 
encoding set, commonly referred to as IBM Code Page 437.  This limits storing 
file name characters to only those within the original MS-DOS range of values 
and does not properly support file names in other character encodings, or 
languages. To address this limitation, this specification will support the 
following change. 

D.2 If general purpose bit 11 is unset, the file name and comment SHOULD conform 
to the original ZIP character encoding.  If general purpose bit 11 is set, the 
filename and comment MUST support The Unicode Standard, Version 4.1.0 or 
greater using the character encoding form defined by the UTF-8 storage 
specification.  The Unicode Standard is published by the The Unicode
Consortium (www.unicode.org).  UTF-8 encoded data stored within ZIP files 
is expected to not include a byte order mark (BOM). 

D.3 Applications MAY choose to supplement this file name storage through the use 
of the 0x0008 Extra Field.  Storage for this optional field is currently 
undefined, however it will be used to allow storing extended information 
on source or target encoding that MAY further assist applications with file 
name, or file content encoding tasks.  Please contact PKWARE with any
requirements on how this field SHOULD be used.

D.4 The 0x0008 Extra Field storage MAY be used with either setting for general 
purpose bit 11.  Examples of the intended usage for this field is to store 
whether "modified-UTF-8" (JAVA) is used, or UTF-8-MAC.  Similarly, other 
commonly used character encoding (code page) designations can be indicated 
through this field.  Formalized values for use of the 0x0008 record remain 
undefined at this time.  The definition for the layout of the 0x0008 field
will be published when available.  Use of the 0x0008 Extra Field provides
for storing data within a ZIP file in an encoding other than IBM Code
Page 437 or UTF-8.

D.5 General purpose bit 11 will not imply any encoding of file content or
password.  Values defining character encoding for file content or 
password MUST be stored within the 0x0008 Extended Language Encoding 
Extra Field.

D.6 Ed Gordon of the Info-ZIP group has defined a pair of "extra field" records 
that can be used to store UTF-8 file name and file comment fields.  These
records can be used for cases when the general purpose bit 11 method
for storing UTF-8 data in the standard file name and comment fields is
not desirable.  A common case for this alternate method is if backward
compatibility with older programs is required.

D.7 Definitions for the record structure of these fields are included above 
in the section on 3rd party mappings for "extra field" records.  These
records are identified by Header ID's 0x6375 (Info-ZIP Unicode Comment 
Extra Field) and 0x7075 (Info-ZIP Unicode Path Extra Field).

D.8 The choice of which storage method to use when writing a ZIP file is left
to the implementation.  Developers SHOULD expect that a ZIP file MAY 
contain either method and SHOULD provide support for reading data in 
either format. Use of general purpose bit 11 reduces storage requirements 
for file name data by not requiring additional "extra field" data for
each file, but can result in older ZIP programs not being able to extract 
files.  Use of the 0x6375 and 0x7075 records will result in a ZIP file 
that SHOULD always be readable by older ZIP programs, but requires more 
storage per file to write file name and/or file comment fields.

APPENDIX E - AE-x encryption marker
-----------------------------------

E.1 AE-x defines an alternate password-based encryption method used 
in ZIP files that is based on a file encryption utility developed by 
Dr. Brian Gladman.  Information on Dr. Gladman's method is available at 

   http://www.gladman.me.uk/cryptography_technology/fileencrypt/

E.2 AE-x uses AES with CTR (counter mode) and HMAC-SHA1.  It defines
encryption using key sizes of 128 bits or 256 bits.  It does not 
restrict support for decrypting 192 bits.

E.3 This method uses the standard ZIP encryption bit (bit 0) 
of the general purpose bit flag (section 4.4.4) to indicate a
file is encrypted.  

E.4 The compression method field (section 4.4.5) is set to 99 
to indicate a file has been encrypted using this method.

E.5 The actual compression method is stored in an extra field
structure identified by a Header ID of 0x9901. Information on this
record structure can be found at http://www.winzip.com/aes_info.htm.

E.6 Two versions are defined for the 0x9901 structure.  

   E.6.1 Version 1 stores the file CRC value in the CRC-32 field
   (section 4.4.7). 

   E.6.2 Version 2 stores a value of 0 in the CRC-32 field.

```

