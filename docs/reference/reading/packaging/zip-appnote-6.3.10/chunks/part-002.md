# ZIP File Format Specification — part 2

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md) · [Next](part-003.md)

<a id="source-page-1-part-2"></a>

## Source page 1 (continued, part 2)

```text
   empty ZIP file.  Files MAY be added or replaced within a ZIP file, or deleted. 
   A ZIP file MUST have only one "end of central directory record".  Other 
   records defined in this specification MAY be used as needed to support 
   storage requirements for individual ZIP files.

   4.3.2 Each file placed into a ZIP file MUST be preceded by  a "local 
   file header" record for that file.  Each "local file header" MUST be 
   accompanied by a corresponding "central directory header" record within 
   the central directory section of the ZIP file.

   4.3.3 Files MAY be stored in arbitrary order within a ZIP file.  A ZIP 
   file MAY span multiple volumes or it MAY be split into user-defined 
   segment sizes. All values MUST be stored in little-endian byte order unless 
   otherwise specified in this document for a specific data element. 

   4.3.4 Compression MUST NOT be applied to a "local file header", an "encryption
   header", or an "end of central directory record".  Individual "central 
   directory records" MUST NOT be compressed, but the aggregate of all central
   directory records MAY be compressed.    

   4.3.5 File data MAY be followed by a "data descriptor" for the file.  Data 
   descriptors are used to facilitate ZIP file streaming.  

 
   4.3.6 Overall .ZIP file format:

      [local file header 1]
      [encryption header 1]
      [file data 1]
      [data descriptor 1]
      . 
      .
      .
      [local file header n]
      [encryption header n]
      [file data n]
      [data descriptor n]
      [archive decryption header] 
      [archive extra data record] 
      [central directory header 1]
      .
      .
      .
      [central directory header n]
      [zip64 end of central directory record]
      [zip64 end of central directory locator] 
      [end of central directory record]


   4.3.7  Local file header:

      local file header signature     4 bytes  (0x04034b50)
      version needed to extract       2 bytes
      general purpose bit flag        2 bytes
      compression method              2 bytes
      last mod file time              2 bytes
      last mod file date              2 bytes
      crc-32                          4 bytes
      compressed size                 4 bytes
      uncompressed size               4 bytes
      file name length                2 bytes
      extra field length              2 bytes

      file name (variable size)
      extra field (variable size)

   4.3.8  File data

      Immediately following the local header for a file
      SHOULD be placed the compressed or stored data for the file.
      If the file is encrypted, the encryption header for the file 
      SHOULD be placed after the local header and before the file 
      data. The series of [local file header][encryption header]
      [file data][data descriptor] repeats for each file in the 
      .ZIP archive. 

      Zero-byte files, directories, and other file types that 
      contain no content MUST NOT include file data.

   4.3.9  Data descriptor:

        crc-32                          4 bytes
        compressed size                 4 bytes
        uncompressed size               4 bytes

      4.3.9.1 This descriptor MUST exist if bit 3 of the general
      purpose bit flag is set (see below).  It is byte aligned
      and immediately follows the last byte of compressed data.
      This descriptor SHOULD be used only when it was not possible to
      seek in the output .ZIP file, e.g., when the output .ZIP file
      was standard output or a non-seekable device.  For ZIP64(tm) format
      archives, the compressed and uncompressed sizes are 8 bytes each.

      4.3.9.2 When compressing files, compressed and uncompressed sizes 
      SHOULD be stored in ZIP64 format (as 8 byte values) when a 
      file's size exceeds 0xFFFFFFFF.   However ZIP64 format MAY be 
      used regardless of the size of a file.  When extracting, if 
      the zip64 extended information extra field is present for 
      the file the compressed and uncompressed sizes will be 8
      byte values.  

      4.3.9.3 Although not originally assigned a signature, the value 
      0x08074b50 has commonly been adopted as a signature value 
      for the data descriptor record.  Implementers SHOULD be 
      aware that ZIP files MAY be encountered with or without this 
      signature marking data descriptors and SHOULD account for
      either case when reading ZIP files to ensure compatibility.

      4.3.9.4 When writing ZIP files, implementors SHOULD include the
      signature value marking the data descriptor record.  When
      the signature is used, the fields currently defined for
      the data descriptor record will immediately follow the
      signature.

      4.3.9.5 An extensible data descriptor will be released in a 
      future version of this APPNOTE.  This new record is intended to
      resolve conflicts with the use of this record going forward,
      and to provide better support for streamed file processing.

      4.3.9.6 When the Central Directory Encryption method is used, 
      the data descriptor record is not required, but MAY be used.  
      If present, and bit 3 of the general purpose bit field is set to 
      indicate its presence, the values in fields of the data descriptor
      record MUST be set to binary zeros.  See the section on the Strong 
      Encryption Specification for information. Refer to the section in 
      this document entitled "Incorporating PKWARE Proprietary Technology 
      into Your Product" for more information.


   4.3.10  Archive decryption header:  

      4.3.10.1 The Archive Decryption Header is introduced in version 6.2
      of the ZIP format specification.  This record exists in support
      of the Central Directory Encryption Feature implemented as part of 
      the Strong Encryption Specification as described in this document.
      When the Central Directory Structure is encrypted, this decryption
      header MUST precede the encrypted data segment.  

      4.3.10.2 The encrypted data segment SHALL consist of the Archive 
      extra data record (if present) and the encrypted Central Directory 
      Structure data.  The format of this data record is identical to the 
      Decryption header record preceding compressed file data.  If the 
      central directory structure is encrypted, the location of the start of
      this data record is determined using the Start of Central Directory
      field in the Zip64 End of Central Directory record.  See the 
      section on the Strong Encryption Specification for information
      on the fields used in the Archive Decryption Header record.
      Refer to the section in this document entitled "Incorporating 
      PKWARE Proprietary Technology into Your Product" for more information.


   4.3.11  Archive extra data record: 

        archive extra data signature    4 bytes  (0x08064b50)
        extra field length              4 bytes
        extra field data                (variable size)

      4.3.11.1 The Archive Extra Data Record is introduced in version 6.2
      of the ZIP format specification.  This record MAY be used in support
      of the Central Directory Encryption Feature implemented as part of 
      the Strong Encryption Specification as described in this document.
      When present, this record MUST immediately precede the central 
      directory data structure.  

      4.3.11.2 The size of this data record SHALL be included in the 
      Size of the Central Directory field in the End of Central 
      Directory record.  If the central directory structure is compressed, 
      but not encrypted, the location of the start of this data record is 
      determined using the Start of Central Directory field in the Zip64 
      End of Central Directory record. Refer to the section in this document 
      entitled "Incorporating PKWARE Proprietary Technology into Your 
      Product" for more information.

   4.3.12  Central directory structure:

      [central directory header 1]
      .
      .
      . 
      [central directory header n]
      [digital signature] 

      File header:

        central file header signature   4 bytes  (0x02014b50)
        version made by                 2 bytes
        version needed to extract       2 bytes
        general purpose bit flag        2 bytes
        compression method              2 bytes
        last mod file time              2 bytes
        last mod file date              2 bytes
        crc-32                          4 bytes
        compressed size                 4 bytes
        uncompressed size               4 bytes
        file name length                2 bytes
        extra field length              2 bytes
        file comment length             2 bytes
        disk number start               2 bytes
        internal file attributes        2 bytes
        external file attributes        4 bytes
        relative offset of local header 4 bytes

        file name (variable size)
        extra field (variable size)
        file comment (variable size)

   4.3.13 Digital signature:

        header signature                4 bytes  (0x05054b50)
        size of data                    2 bytes
        signature data (variable size)

      With the introduction of the Central Directory Encryption 
      feature in version 6.2 of this specification, the Central 
      Directory Structure MAY be stored both compressed and encrypted. 
      Although not required, it is assumed when encrypting the
      Central Directory Structure, that it will be compressed
      for greater storage efficiency.  Information on the
      Central Directory Encryption feature can be found in the section
      describing the Strong Encryption Specification. The Digital 
      Signature record will be neither compressed nor encrypted.

   4.3.14  Zip64 end of central directory record

        zip64 end of central dir 
        signature                       4 bytes  (0x06064b50)
        size of zip64 end of central
        directory record                8 bytes
        version made by                 2 bytes
        version needed to extract       2 bytes
        number of this disk             4 bytes
        number of the disk with the 
        start of the central directory  4 bytes
        total number of entries in the
        central directory on this disk  8 bytes
        total number of entries in the
        central directory               8 bytes
        size of the central directory   8 bytes
        offset of start of central
        directory with respect to
        the starting disk number        8 bytes
        zip64 extensible data sector    (variable size)

      4.3.14.1 The value stored into the "size of zip64 end of central
      directory record" SHOULD be the size of the remaining
      record and SHOULD NOT include the leading 12 bytes.
  
      Size = SizeOfFixedFields + SizeOfVariableData - 12.

      4.3.14.2 The above record structure defines Version 1 of the 
      zip64 end of central directory record. Version 1 was 
      implemented in versions of this specification preceding 
      6.2 in support of the ZIP64 large file feature. The 
      introduction of the Central Directory Encryption feature 
      implemented in version 6.2 as part of the Strong Encryption 
      Specification defines Version 2 of this record structure. 
      Refer to the section describing the Strong Encryption 
      Specification for details on the version 2 format for 
      this record. Refer to the section in this document entitled 
      "Incorporating PKWARE Proprietary Technology into Your Product"
      for more information applicable to use of Version 2 of this
      record.

      4.3.14.3 Special purpose data MAY reside in the zip64 extensible 
      data sector field following either a V1 or V2 version of this
      record.  To ensure identification of this special purpose data
      it MUST include an identifying header block consisting of the
      following:

         Header ID  -  2 bytes
         Data Size  -  4 bytes

      The Header ID field indicates the type of data that is in the 
      data block that follows.

      Data Size identifies the number of bytes that follow for this
      data block type.

      4.3.14.4 Multiple special purpose data blocks MAY be present. 
      Each MUST be preceded by a Header ID and Data Size field.  Current
      mappings of Header ID values supported in this field are as
      defined in APPENDIX C.

   4.3.15 Zip64 end of central directory locator

      zip64 end of central dir locator 
      signature                       4 bytes  (0x07064b50)
      number of the disk with the
      start of the zip64 end of 
      central directory               4 bytes
      relative offset of the zip64
      end of central directory record 8 bytes
      total number of disks           4 bytes
        
   4.3.16  End of central directory record:

      end of central dir signature    4 bytes  (0x06054b50)
      number of this disk             2 bytes
      number of the disk with the
      start of the central directory  2 bytes
      total number of entries in the
      central directory on this disk  2 bytes
      total number of entries in
      the central directory           2 bytes
      size of the central directory   4 bytes
      offset of start of central
      directory with respect to
      the starting disk number        4 bytes
      .ZIP file comment length        2 bytes
      .ZIP file comment       (variable size)
                
4.4  Explanation of fields
--------------------------
```

