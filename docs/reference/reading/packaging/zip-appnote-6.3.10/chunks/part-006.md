# ZIP File Format Specification — part 6

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-005.md) · [Next](part-007.md)

<a id="source-page-1-part-6"></a>

## Source page 1 (continued, part 6)

```text
          0xd935        Android ZIP Alignment Extra Field        
          0xe57a        Korean ZIP code page info
          0xfd4a        SMS/QDOS
          0x9901        AE-x encryption structure (see APPENDIX E)
          0x9902        unknown


   Detailed descriptions of Extra Fields defined by third 
   party mappings will be documented as information on
   these data structures is made available to PKWARE.  
   PKWARE does not guarantee the accuracy of any published
   third party data.

   4.6.2 Third-party Extra Fields MUST include a Header ID using
   the format defined in the section of this document 
   titled Extensible Data Fields (section 4.5).

   The Data Size field indicates the size of the following
   data block. Programs can use this value to skip to the
   next header block, passing over any data blocks that are
   not of interest.

   Note: As stated above, the size of the entire .ZIP file
         header, including the file name, comment, and extra
         field SHOULD NOT exceed 64K in size.

   4.6.3 In case two different programs appropriate the same
   Header ID value, it is strongly recommended that each
   program SHOULD place a unique signature of at least two bytes in
   size (and preferably 4 bytes or bigger) at the start of
   each data area.  Every program SHOULD verify that its
   unique signature is present, in addition to the Header ID
   value being correct, before assuming that it is a block of
   known type.
         
   Third-party Mappings:
   Not all third-party extra field mappings are documented here.
          
   4.6.4 -ZipIt Macintosh Extra Field (long) (0x2605):

      The following is the layout of the ZipIt extra block 
      for Macintosh. The local-header and central-header versions 
      are identical. This block MUST be present if the file is 
      stored MacBinary-encoded and it SHOULD NOT be used if the file 
      is not stored MacBinary-encoded.

          Value         Size        Description
          -----         ----        -----------
  (Mac2)  0x2605        Short       tag for this extra block type
          TSize         Short       total data size for this block
          "ZPIT"        beLong      extra-field signature
          FnLen         Byte        length of FileName
          FileName      variable    full Macintosh filename
          FileType      Byte[4]     four-byte Mac file type string
          Creator       Byte[4]     four-byte Mac creator string


   4.6.5 -ZipIt Macintosh Extra Field (short, for files) (0x2705):

      The following is the layout of a shortened variant of the
      ZipIt extra block for Macintosh (without "full name" entry).
      This variant is used by ZipIt 1.3.5 and newer for entries of
      files (not directories) that do not have a MacBinary encoded
      file. The local-header and central-header versions are identical.

         Value         Size        Description
         -----         ----        -----------
 (Mac2b) 0x2705        Short       tag for this extra block type
         TSize         Short       total data size for this block (12)
         "ZPIT"        beLong      extra-field signature
         FileType      Byte[4]     four-byte Mac file type string
         Creator       Byte[4]     four-byte Mac creator string
         fdFlags       beShort     attributes from FInfo.frFlags,
                                   MAY be omitted
         0x0000        beShort     reserved, MAY be omitted


   4.6.6 -ZipIt Macintosh Extra Field (short, for directories) (0x2805):

      The following is the layout of a shortened variant of the
      ZipIt extra block for Macintosh used only for directory
      entries. This variant is used by ZipIt 1.3.5 and newer to 
      save some optional Mac-specific information about directories.
      The local-header and central-header versions are identical.

         Value         Size        Description
         -----         ----        -----------
 (Mac2c) 0x2805        Short       tag for this extra block type
         TSize         Short       total data size for this block (12)
         "ZPIT"        beLong      extra-field signature
         frFlags       beShort     attributes from DInfo.frFlags, MAY
                                   be omitted
         View          beShort     ZipIt view flag, MAY be omitted


     The View field specifies ZipIt-internal settings as follows:

     Bits of the Flags:
        bit 0           if set, the folder is shown expanded (open)
                        when the archive contents are viewed in ZipIt.
        bits 1-15       reserved, zero;


   4.6.7 -FWKCS MD5 Extra Field (0x4b46):

      The FWKCS Contents_Signature System, used in
      automatically identifying files independent of file name,
      optionally adds and uses an extra field to support the
      rapid creation of an enhanced contents_signature:

              Header ID = 0x4b46
              Data Size = 0x0013
              Preface   = 'M','D','5'
              followed by 16 bytes containing the uncompressed file's
              128_bit MD5 hash(1), low byte first.

      When FWKCS revises a .ZIP file central directory to add
      this extra field for a file, it also replaces the
      central directory entry for that file's uncompressed
      file length with a measured value.

      FWKCS provides an option to strip this extra field, if
      present, from a .ZIP file central directory. In adding
      this extra field, FWKCS preserves .ZIP file Authenticity
      Verification; if stripping this extra field, FWKCS
      preserves all versions of AV through PKZIP version 2.04g.

      FWKCS, and FWKCS Contents_Signature System, are
      trademarks of Frederick W. Kantor.

      (1) R. Rivest, RFC1321.TXT, MIT Laboratory for Computer
          Science and RSA Data Security, Inc., April 1992.
          ll.76-77: "The MD5 algorithm is being placed in the
          public domain for review and possible adoption as a
          standard."


   4.6.8 -Info-ZIP Unicode Comment Extra Field (0x6375):

      Stores the UTF-8 version of the file comment as stored in the
      central directory header. (Last Revision 20070912)

         Value         Size        Description
         -----         ----        -----------
  (UCom) 0x6375        Short       tag for this extra block type ("uc")
         TSize         Short       total data size for this block
         Version       1 byte      version of this extra field, currently 1
         ComCRC32      4 bytes     Comment Field CRC32 Checksum
         UnicodeCom    Variable    UTF-8 version of the entry comment

       Currently Version is set to the number 1.  If there is a need
       to change this field, the version will be incremented.  Changes
       MAY NOT be backward compatible so this extra field SHOULD NOT be
       used if the version is not recognized.

       The ComCRC32 is the standard zip CRC32 checksum of the File Comment
       field in the central directory header.  This is used to verify that
       the comment field has not changed since the Unicode Comment extra field
       was created.  This can happen if a utility changes the File Comment 
       field but does not update the UTF-8 Comment extra field.  If the CRC 
       check fails, this Unicode Comment extra field SHOULD be ignored and 
       the File Comment field in the header SHOULD be used instead.

       The UnicodeCom field is the UTF-8 version of the File Comment field
       in the header.  As UnicodeCom is defined to be UTF-8, no UTF-8 byte
       order mark (BOM) is used.  The length of this field is determined by
       subtracting the size of the previous fields from TSize.  If both the
       File Name and Comment fields are UTF-8, the new General Purpose Bit
       Flag, bit 11 (Language encoding flag (EFS)), can be used to indicate
       both the header File Name and Comment fields are UTF-8 and, in this
       case, the Unicode Path and Unicode Comment extra fields are not
       needed and SHOULD NOT be created.  Note that, for backward
       compatibility, bit 11 SHOULD only be used if the native character set
       of the paths and comments being zipped up are already in UTF-8. It is
       expected that the same file comment storage method, either general
       purpose bit 11 or extra fields, be used in both the Local and Central
       Directory Header for a file.


   4.6.9 -Info-ZIP Unicode Path Extra Field (0x7075):

       Stores the UTF-8 version of the file name field as stored in the
       local header and central directory header. (Last Revision 20070912)

         Value         Size        Description
         -----         ----        -----------
 (UPath) 0x7075        Short       tag for this extra block type ("up")
         TSize         Short       total data size for this block
         Version       1 byte      version of this extra field, currently 1
         NameCRC32     4 bytes     File Name Field CRC32 Checksum
         UnicodeName   Variable    UTF-8 version of the entry File Name

      Currently Version is set to the number 1.  If there is a need
      to change this field, the version will be incremented.  Changes
      MAY NOT be backward compatible so this extra field SHOULD NOT be
      used if the version is not recognized.

      The NameCRC32 is the standard zip CRC32 checksum of the File Name
      field in the header.  This is used to verify that the header
      File Name field has not changed since the Unicode Path extra field
      was created.  This can happen if a utility renames the File Name but
      does not update the UTF-8 path extra field.  If the CRC check fails,
      this UTF-8 Path Extra Field SHOULD be ignored and the File Name field
      in the header SHOULD be used instead.

      The UnicodeName is the UTF-8 version of the contents of the File Name
      field in the header.  As UnicodeName is defined to be UTF-8, no UTF-8
      byte order mark (BOM) is used.  The length of this field is determined
      by subtracting the size of the previous fields from TSize.  If both
      the File Name and Comment fields are UTF-8, the new General Purpose
      Bit Flag, bit 11 (Language encoding flag (EFS)), can be used to
      indicate that both the header File Name and Comment fields are UTF-8
      and, in this case, the Unicode Path and Unicode Comment extra fields
      are not needed and SHOULD NOT be created.  Note that, for backward
      compatibility, bit 11 SHOULD only be used if the native character set
      of the paths and comments being zipped up are already in UTF-8. It is
      expected that the same file name storage method, either general
      purpose bit 11 or extra fields, be used in both the Local and Central
      Directory Header for a file.
 

   4.6.10 -Microsoft Open Packaging Growth Hint (0xa220):

          Value         Size        Description
          -----         ----        -----------
          0xa220        Short       tag for this extra block type
          TSize         Short       size of Sig + PadVal + Padding
          Sig           Short       verification signature (A028)
          PadVal        Short       Initial padding value
          Padding       variable    filled with NULL characters

    4.6.11 -Data Stream Alignment (Apache Commons-Compress) (0xa11e):

       (per Zbynek Vyskovsky) Defines alignment of data stream of this 
       entry within the zip archive.  Additionally, indicates whether the 
       compression method should be kept when re-compressing the zip file.

       The purpose of this extra field is to align specific resources to 
       word or page boundaries so they can be easily mapped into memory.  

         Value         Size        Description
         -----         ----        -----------
         0xa11e        Short       tag for this extra block type
         TSize         Short       total data size for this block (2+padding)
         alignment     Short       required alignment and indicator
         0x00          Variable    padding

       The alignment field (lower 15 bits) defines the minimal alignment 
       required by the data stream.   Bit 15 of alignment field indicates 
       whether the compression method of this entry can be changed when 
       recompressing the zip file.  The value 0 means the compression method 
       should not be changed.  The value 1 indicates  the compression method 
       may be changed. The padding field contains padding to ensure the correct 
       alignment.  It can be changed at any time when the offset or required 
       alignment changes. (see https://issues.apache.org/jira/browse/COMPRESS-391)


4.7 Manifest Files
------------------

    4.7.1 Applications using ZIP files MAY have a need for additional 
    information that MUST be included with the files placed into
    a ZIP file. Application specific information that cannot be
    stored using the defined ZIP storage records SHOULD be stored 
    using the extensible Extra Field convention defined in this 
    document.  However, some applications MAY use a manifest
    file as a means for storing additional information.  One
    example is the META-INF/MANIFEST.MF file used in ZIP formatted
    files having the .JAR extension (JAR files).  

    4.7.2 A manifest file is a file created for the application process
    that requires this information.  A manifest file MAY be of any 
    file type required by the defining application process.  It is 
    placed within the same ZIP file as files to which this information 
    applies. By convention, this file is typically the first file placed
```

