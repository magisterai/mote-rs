# ZIP File Format Specification — part 1

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Next](part-002.md)

<a id="source-page-1-part-1"></a>

## Source page 1

```text
File:    APPNOTE.TXT - .ZIP File Format Specification
Version: 6.3.10 
Status: FINAL - replaces version 6.3.9
Revised: Nov 01, 2022
Copyright (c) 1989 - 2014, 2018, 2019, 2020, 2022 PKWARE Inc., All Rights Reserved.

1.0 Introduction
---------------

1.1 Purpose
-----------

   1.1.1 This specification is intended to define a cross-platform,
   interoperable file storage and transfer format.  Since its 
   first publication in 1989, PKWARE, Inc. ("PKWARE") has remained 
   committed to ensuring the interoperability of the .ZIP file 
   format through periodic publication and maintenance of this 
   specification.  We trust that all .ZIP compatible vendors and 
   application developers that use and benefit from this format 
   will share and support this commitment to interoperability.

1.2 Scope
---------

   1.2.1 ZIP is one of the most widely used compressed file formats. It is 
   universally used to aggregate, compress, and encrypt files into a single
   interoperable container. No specific use or application need is 
   defined by this format and no specific implementation guidance is 
   provided. This document provides details on the storage format for 
   creating ZIP files.  Information is provided on the records and 
   fields that describe what a ZIP file is. 

1.3 Trademarks
--------------

   1.3.1 PKWARE, PKZIP, Smartcrypt, SecureZIP, and PKSFX are registered 
   trademarks of PKWARE, Inc. in the United States and elsewhere.  
   PKPatchMaker, Deflate64, and ZIP64 are trademarks of PKWARE, Inc.  
   Other marks referenced within this document appear for identification
   purposes only and are the property of their respective owners.
   

1.4 Permitted Use
----------------- 

   1.4.1 This document, "APPNOTE.TXT -  .ZIP File Format Specification" is the
   exclusive property of PKWARE.  Use of the information contained in this 
   document is permitted solely for the purpose of creating products, 
   programs and processes that read and write files in the ZIP format
   subject to the terms and conditions herein.

   1.4.2 Use of the content of this document within other publications is 
   permitted only through reference to this document.  Any reproduction
   or distribution of this document in whole or in part without prior
   written permission from PKWARE is strictly prohibited.

   1.4.3 Certain technological components provided in this document are the 
   patented proprietary technology of PKWARE and as such require a 
   separate, executed license agreement from PKWARE.  Applicable 
   components are marked with the following, or similar, statement: 
   'Refer to the section in this document entitled  "Incorporating 
   PKWARE Proprietary Technology into Your Product" for more information'.

1.5 Contacting PKWARE
---------------------

   1.5.1 If you have questions on this format, its use, or licensing, or if you 
   wish to report defects, request changes or additions, please contact:

     PKWARE, Inc.
     201 E. Pittsburgh Avenue, Suite 400
     Milwaukee, WI 53204
     +1-414-289-9788
     +1-414-289-9789 FAX
     zipformat@pkware.com

   1.5.2 Information about this format and a reference copy of this document
   is publicly available at:

     http://www.pkware.com/appnote

1.6 Disclaimer
--------------

   1.6.1 Although PKWARE will attempt to supply current and accurate
   information relating to its file formats, algorithms, and the
   subject programs, the possibility of error or omission cannot 
   be eliminated. PKWARE therefore expressly disclaims any warranty 
   that the information contained in the associated materials relating 
   to the subject programs and/or the format of the files created or
   accessed by the subject programs and/or the algorithms used by
   the subject programs, or any other matter, is current, correct or
   accurate as delivered.  Any risk of damage due to any possible
   inaccurate information is assumed by the user of the information.
   Furthermore, the information relating to the subject programs
   and/or the file formats created or accessed by the subject
   programs and/or the algorithms used by the subject programs is
   subject to change without notice.

2.0 Revisions
--------------

2.1 Document Status
--------------------

   2.1.1 If the STATUS of this file is marked as DRAFT, the content 
   defines proposed revisions to this specification which may consist 
   of changes to the ZIP format itself, or that may consist of other 
   content changes to this document.  Versions of this document and 
   the format in DRAFT form may be subject to modification prior to 
   publication STATUS of FINAL. DRAFT versions are published periodically 
   to provide notification to the ZIP community of pending changes and to 
   provide opportunity for review and comment.

   2.1.2 Versions of this document having a STATUS of FINAL are 
   considered to be in the final form for that version of the document 
   and are not subject to further change until a new, higher version
   numbered document is published.  Newer versions of this format 
   specification are intended to remain interoperable with all prior 
   versions whenever technically possible.  

2.2 Change Log
--------------

   Version       Change Description                        Date
   -------       ------------------                       ----------
   5.2           -Single Password Symmetric Encryption    07/16/2003
                  storage

   6.1.0         -Smartcard compatibility                 01/20/2004
                 -Documentation on certificate storage

   6.2.0         -Introduction of Central Directory       04/26/2004
                  Encryption for encrypting metadata
                 -Added OS X to Version Made By values

   6.2.1         -Added Extra Field placeholder for       04/01/2005
                  POSZIP using ID 0x4690

                 -Clarified size field on 
                  "zip64 end of central directory record"

   6.2.2         -Documented Final Feature Specification  01/06/2006
                  for Strong Encryption

                 -Clarifications and typographical 
                  corrections

   6.3.0         -Added tape positioning storage          09/29/2006
                  parameters

                 -Expanded list of supported hash algorithms

                 -Expanded list of supported compression
                  algorithms

                 -Expanded list of supported encryption
                  algorithms

                 -Added option for Unicode filename 
                  storage

                 -Clarifications for consistent use
                  of Data Descriptor records

                 -Added additional "Extra Field" 
                  definitions

   6.3.1         -Corrected standard hash values for      04/11/2007
                  SHA-256/384/512

   6.3.2         -Added compression method 97             09/28/2007

                 -Documented InfoZIP "Extra Field"
                  values for UTF-8 file name and
                  file comment storage

   6.3.3         -Formatting changes to support           09/01/2012
                  easier referencing of this APPNOTE
                  from other documents and standards  

   6.3.4         -Address change                          10/01/2014
 
   6.3.5         -Documented compression methods 16       11/31/2018
                  and 99 (4.4.5, 4.6.1, 5.11, 5.17, 
                  APPENDIX E)

                 -Corrected several typographical 
                  errors (2.1.2, 3.2, 4.1.1, 10.2)
                 
                 -Marked legacy algorithms as no
                  longer suitable for use (4.4.5.1)

                 -Added clarity on MS DOS time format
                  (4.4.6)

                 -Assign extrafield ID for Timestamps
                  (4.5.2)

                 -Field code description correction (A.2)

                 -More consistent use of MAY/SHOULD/MUST

                 -Expanded 0x0065 record attribute codes (B.2)

                 -Initial information on 0x0022 Extra Data

   6.3.6         -Corrected typographical error          04/26/2019
                  (4.4.1.3) 

   6.3.7         -Added Zstandard compression method ID  
                  (4.4.5)

                 -Corrected several reported typos

                 -Marked intended use for general purpose bit 14
 
                 -Added Data Stream Alignment Extra Data info
                  (4.6.11)

   6.3.8         -Resolved Zstandard compression method ID conflict  
                  (4.4.5)

                 -Added additional compression method ID values in use

   6.3.9         -Corrected a typo in Data Stream Alignment description
                  (4.6.11)

   6.3.10        -Added several z/OS attribute values for APPENDIX B

                 -Added several additional 3rd party Extra Field mappings
                  (thanks to Armijn Hemel @tjaldur.nl for forwarding info
                  on several of the Header ID's)



3.0 Notations
-------------

   3.1 Use of the term MUST or SHALL indicates a required element. 

   3.2 MUST NOT or SHALL NOT indicates an element is prohibited from use. 

   3.3 SHOULD indicates a RECOMMENDED element.

   3.4 SHOULD NOT indicates an element NOT RECOMMENDED for use.
   
   3.5 MAY indicates an OPTIONAL element.


4.0 ZIP Files
-------------

4.1 What is a ZIP file
----------------------

   4.1.1 ZIP files MAY be identified by the standard .ZIP file extension 
   although use of a file extension is not required.  Use of the 
   extension .ZIPX is also recognized and MAY be used for ZIP files.  
   Other common file extensions using the ZIP format include .JAR, .WAR, 
   .DOCX, .XLSX, .PPTX, .ODT, .ODS, .ODP and others. Programs reading or 
   writing ZIP files SHOULD rely on internal record signatures described 
   in this document to identify files in this format.

   4.1.2 ZIP files SHOULD contain at least one file and MAY contain 
   multiple files.  

   4.1.3 Data compression MAY be used to reduce the size of files
   placed into a ZIP file, but is not required.  This format supports the 
   use of multiple data compression algorithms.  When compression is used, 
   one of the documented compression algorithms MUST be used.  Implementors 
   are advised to experiment with their data to determine which of the 
   available algorithms provides the best compression for their needs.
   Compression method 8 (Deflate) is the method used by default by most 
   ZIP compatible application programs.  


   4.1.4 Data encryption MAY be used to protect files within a ZIP file. 
   Keying methods supported for encryption within this format include
   passwords and public/private keys.  Either MAY be used individually
   or in combination. Encryption MAY be applied to individual files.  
   Additional security MAY be used through the encryption of ZIP file 
   metadata stored within the Central Directory. See the section on the 
   Strong Encryption Specification for information. Refer to the section 
   in this document entitled "Incorporating PKWARE Proprietary Technology 
   into Your Product" for more information.

   4.1.5 Data integrity MUST be provided for each file using CRC32.  
   
   4.1.6 Additional data integrity MAY be included through the use of 
   digital signatures.  Individual files MAY be signed with one or more 
   digital signatures. The Central Directory, if signed, MUST use a 
   single signature.  

   4.1.7 Files MAY be placed within a ZIP file uncompressed or stored. 
   The term "stored" as used in the context of this document means the file 
   is copied into the ZIP file uncompressed.  

   4.1.8 Each data file placed into a ZIP file MAY be compressed, stored, 
   encrypted or digitally signed independent of how other data files in the 
   same ZIP file are archived.

   4.1.9 ZIP files MAY be streamed, split into segments (on fixed or on
   removable media) or "self-extracting".  Self-extracting ZIP 
   files MUST include extraction code for a target platform within 
   the ZIP file.  

   4.1.10 Extensibility is provided for platform or application specific
   needs through extra data fields that MAY be defined for custom
   purposes.  Extra data definitions MUST NOT conflict with existing
   documented record definitions.  

   4.1.11 Common uses for ZIP MAY also include the use of manifest files.  
   Manifest files store application specific information within a file stored 
   within the ZIP file.  This manifest file SHOULD be the first file in the 
   ZIP file. This specification does not provide any information or guidance on 
   the use of manifest files within ZIP files.  Refer to the application developer
   for information on using manifest files and for any additional profile
   information on using ZIP within an application.

   4.1.12 ZIP files MAY be placed within other ZIP files.

4.2 ZIP Metadata
----------------

   4.2.1 ZIP files are identified by metadata consisting of defined record types 
   containing the storage information necessary for maintaining the files 
   placed into a ZIP file.  Each record type MUST be identified using a header 
   signature that identifies the record type.  Signature values begin with the 
   two byte constant marker of 0x4b50, representing the characters "PK".


4.3 General Format of a .ZIP file
---------------------------------

   4.3.1 A ZIP file MUST contain an "end of central directory record". A ZIP 
   file containing only an "end of central directory record" is considered an 
```

