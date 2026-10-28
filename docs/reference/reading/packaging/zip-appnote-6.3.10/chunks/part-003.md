# ZIP File Format Specification — part 3

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-002.md) · [Next](part-004.md)

<a id="source-page-1-part-3"></a>

## Source page 1 (continued, part 3)

```text
      
   4.4.1 General notes on fields

      4.4.1.1  All fields unless otherwise noted are unsigned and stored
      in Intel low-byte:high-byte, low-word:high-word order.

      4.4.1.2  String fields are not null terminated, since the length 
      is given explicitly.

      4.4.1.3  The entries in the central directory MAY NOT necessarily
      be in the same order that files appear in the .ZIP file.

      4.4.1.4  If one of the fields in the end of central directory
      record is too small to hold required data, the field SHOULD be 
      set to -1 (0xFFFF or 0xFFFFFFFF) and the ZIP64 format record 
      SHOULD be created.

      4.4.1.5  The end of central directory record and the Zip64 end 
      of central directory locator record MUST reside on the same 
      disk when splitting or spanning an archive.

   4.4.2 version made by (2 bytes)

        4.4.2.1 The upper byte indicates the compatibility of the file
        attribute information.  If the external file attributes 
        are compatible with MS-DOS and can be read by PKZIP for 
        DOS version 2.04g then this value will be zero.  If these 
        attributes are not compatible, then this value will 
        identify the host system on which the attributes are 
        compatible.  Software can use this information to determine
        the line record format for text files etc.  

        4.4.2.2 The current mappings are:

         0 - MS-DOS and OS/2 (FAT / VFAT / FAT32 file systems)
         1 - Amiga                     2 - OpenVMS
         3 - UNIX                      4 - VM/CMS
         5 - Atari ST                  6 - OS/2 H.P.F.S.
         7 - Macintosh                 8 - Z-System
         9 - CP/M                     10 - Windows NTFS
        11 - MVS (OS/390 - Z/OS)      12 - VSE
        13 - Acorn Risc               14 - VFAT
        15 - alternate MVS            16 - BeOS
        17 - Tandem                   18 - OS/400
        19 - OS X (Darwin)            20 thru 255 - unused

        4.4.2.3 The lower byte indicates the ZIP specification version 
        (the version of this document) supported by the software 
        used to encode the file.  The value/10 indicates the major 
        version number, and the value mod 10 is the minor version 
        number.  

   4.4.3 version needed to extract (2 bytes)

        4.4.3.1 The minimum supported ZIP specification version needed 
        to extract the file, mapped as above.  This value is based on 
        the specific format features a ZIP program MUST support to 
        be able to extract the file.  If multiple features are
        applied to a file, the minimum version MUST be set to the 
        feature having the highest value. New features or feature 
        changes affecting the published format specification will be 
        implemented using higher version numbers than the last 
        published value to avoid conflict.

        4.4.3.2 Current minimum feature versions are as defined below:

         1.0 - Default value
         1.1 - File is a volume label
         2.0 - File is a folder (directory)
         2.0 - File is compressed using Deflate compression
         2.0 - File is encrypted using traditional PKWARE encryption
         2.1 - File is compressed using Deflate64(tm)
         2.5 - File is compressed using PKWARE DCL Implode 
         2.7 - File is a patch data set 
         4.5 - File uses ZIP64 format extensions
         4.6 - File is compressed using BZIP2 compression*
         5.0 - File is encrypted using DES
         5.0 - File is encrypted using 3DES
         5.0 - File is encrypted using original RC2 encryption
         5.0 - File is encrypted using RC4 encryption
         5.1 - File is encrypted using AES encryption
         5.1 - File is encrypted using corrected RC2 encryption**
         5.2 - File is encrypted using corrected RC2-64 encryption**
         6.1 - File is encrypted using non-OAEP key wrapping***
         6.2 - Central directory encryption
         6.3 - File is compressed using LZMA
         6.3 - File is compressed using PPMd+
         6.3 - File is encrypted using Blowfish
         6.3 - File is encrypted using Twofish

        4.4.3.3 Notes on version needed to extract 

        * Early 7.x (pre-7.2) versions of PKZIP incorrectly set the
        version needed to extract for BZIP2 compression to be 50
        when it SHOULD have been 46.

        ** Refer to the section on Strong Encryption Specification
        for additional information regarding RC2 corrections.

        *** Certificate encryption using non-OAEP key wrapping is the
        intended mode of operation for all versions beginning with 6.1.
        Support for OAEP key wrapping MUST only be used for
        backward compatibility when sending ZIP files to be opened by
        versions of PKZIP older than 6.1 (5.0 or 6.0).

        + Files compressed using PPMd MUST set the version
        needed to extract field to 6.3, however, not all ZIP 
        programs enforce this and MAY be unable to decompress 
        data files compressed using PPMd if this value is set.

        When using ZIP64 extensions, the corresponding value in the
        zip64 end of central directory record MUST also be set.  
        This field SHOULD be set appropriately to indicate whether 
        Version 1 or Version 2 format is in use. 


   4.4.4 general purpose bit flag: (2 bytes)

        Bit 0: If set, indicates that the file is encrypted.

        (For Method 6 - Imploding)
        Bit 1: If the compression method used was type 6,
               Imploding, then this bit, if set, indicates
               an 8K sliding dictionary was used.  If clear,
               then a 4K sliding dictionary was used.

        Bit 2: If the compression method used was type 6,
               Imploding, then this bit, if set, indicates
               3 Shannon-Fano trees were used to encode the
               sliding dictionary output.  If clear, then 2
               Shannon-Fano trees were used.

        (For Methods 8 and 9 - Deflating)
        Bit 2  Bit 1
          0      0    Normal (-en) compression option was used.
          0      1    Maximum (-exx/-ex) compression option was used.
          1      0    Fast (-ef) compression option was used.
          1      1    Super Fast (-es) compression option was used.

        (For Method 14 - LZMA)
        Bit 1: If the compression method used was type 14,
               LZMA, then this bit, if set, indicates
               an end-of-stream (EOS) marker is used to
               mark the end of the compressed data stream.
               If clear, then an EOS marker is not present
               and the compressed data size must be known
               to extract.

        Note:  Bits 1 and 2 are undefined if the compression
               method is any other.

        Bit 3: If this bit is set, the fields crc-32, compressed 
               size and uncompressed size are set to zero in the 
               local header.  The correct values are put in the 
               data descriptor immediately following the compressed
               data.  (Note: PKZIP version 2.04g for DOS only 
               recognizes this bit for method 8 compression, newer 
               versions of PKZIP recognize this bit for any 
               compression method.)

        Bit 4: Reserved for use with method 8, for enhanced
               deflating. 

        Bit 5: If this bit is set, this indicates that the file is 
               compressed patched data.  (Note: Requires PKZIP 
               version 2.70 or greater)

        Bit 6: Strong encryption.  If this bit is set, you MUST
               set the version needed to extract value to at least
               50 and you MUST also set bit 0.  If AES encryption
               is used, the version needed to extract value MUST 
               be at least 51. See the section describing the Strong
               Encryption Specification for details.  Refer to the 
               section in this document entitled "Incorporating PKWARE 
               Proprietary Technology into Your Product" for more 
               information.

        Bit 7: Currently unused.

        Bit 8: Currently unused.

        Bit 9: Currently unused.

        Bit 10: Currently unused.

        Bit 11: Language encoding flag (EFS).  If this bit is set,
                the filename and comment fields for this file
                MUST be encoded using UTF-8. (see APPENDIX D)

        Bit 12: Reserved by PKWARE for enhanced compression.

        Bit 13: Set when encrypting the Central Directory to indicate 
                selected data values in the Local Header are masked to
                hide their actual values.  See the section describing 
                the Strong Encryption Specification for details.  Refer
                to the section in this document entitled "Incorporating 
                PKWARE Proprietary Technology into Your Product" for 
                more information.

        Bit 14: Reserved by PKWARE for alternate streams.

        Bit 15: Reserved by PKWARE.

   4.4.5 compression method: (2 bytes)

        0 - The file is stored (no compression)
        1 - The file is Shrunk
        2 - The file is Reduced with compression factor 1
        3 - The file is Reduced with compression factor 2
        4 - The file is Reduced with compression factor 3
        5 - The file is Reduced with compression factor 4
        6 - The file is Imploded
        7 - Reserved for Tokenizing compression algorithm
        8 - The file is Deflated
        9 - Enhanced Deflating using Deflate64(tm)
       10 - PKWARE Data Compression Library Imploding (old IBM TERSE)
       11 - Reserved by PKWARE
       12 - File is compressed using BZIP2 algorithm
       13 - Reserved by PKWARE
       14 - LZMA
       15 - Reserved by PKWARE
       16 - IBM z/OS CMPSC Compression
       17 - Reserved by PKWARE
       18 - File is compressed using IBM TERSE (new)
       19 - IBM LZ77 z Architecture 
       20 - deprecated (use method 93 for zstd)
       93 - Zstandard (zstd) Compression 
       94 - MP3 Compression 
       95 - XZ Compression 
       96 - JPEG variant
       97 - WavPack compressed data
       98 - PPMd version I, Rev 1
       99 - AE-x encryption marker (see APPENDIX E)

       4.4.5.1 Methods 1-6 are legacy algorithms and are no longer
       recommended for use when compressing files.

   4.4.6 date and time fields: (2 bytes each)

       The date and time are encoded in standard MS-DOS format.
       If input came from standard input, the date and time are
       those at which compression was started for this data. 
       If encrypting the central directory and general purpose bit 
       flag 13 is set indicating masking, the value stored in the 
       Local Header will be zero. MS-DOS time format is different
       from more commonly used computer time formats such as 
       UTC. For example, MS-DOS uses year values relative to 1980
       and 2 second precision.

   4.4.7 CRC-32: (4 bytes)

       The CRC-32 algorithm was generously contributed by
       David Schwaderer and can be found in his excellent
       book "C Programmers Guide to NetBIOS" published by
       Howard W. Sams & Co. Inc.  The 'magic number' for
       the CRC is 0xdebb20e3.  The proper CRC pre and post
       conditioning is used, meaning that the CRC register
       is pre-conditioned with all ones (a starting value
       of 0xffffffff) and the value is post-conditioned by
       taking the one's complement of the CRC residual.
       If bit 3 of the general purpose flag is set, this
       field is set to zero in the local header and the correct
       value is put in the data descriptor and in the central
       directory. When encrypting the central directory, if the
       local header is not in ZIP64 format and general purpose 
       bit flag 13 is set indicating masking, the value stored 
       in the Local Header will be zero. 

   4.4.8 compressed size: (4 bytes)
   4.4.9 uncompressed size: (4 bytes)

       The size of the file compressed (4.4.8) and uncompressed,
       (4.4.9) respectively.  When a decryption header is present it 
       will be placed in front of the file data and the value of the
       compressed file size will include the bytes of the decryption
       header.  If bit 3 of the general purpose bit flag is set, 
       these fields are set to zero in the local header and the 
       correct values are put in the data descriptor and
       in the central directory.  If an archive is in ZIP64 format
       and the value in this field is 0xFFFFFFFF, the size will be
       in the corresponding 8 byte ZIP64 extended information 
       extra field.  When encrypting the central directory, if the
       local header is not in ZIP64 format and general purpose bit 
       flag 13 is set indicating masking, the value stored for the 
       uncompressed size in the Local Header will be zero. 

   4.4.10 file name length: (2 bytes)
   4.4.11 extra field length: (2 bytes)
   4.4.12 file comment length: (2 bytes)

       The length of the file name, extra field, and comment
       fields respectively.  The combined length of any
       directory record and these three fields SHOULD NOT
       generally exceed 65,535 bytes.  If input came from standard
       input, the file name length is set to zero.  


   4.4.13 disk number start: (2 bytes)

       The number of the disk on which this file begins.  If an 
       archive is in ZIP64 format and the value in this field is 
       0xFFFF, the size will be in the corresponding 4 byte zip64 
```

