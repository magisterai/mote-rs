# ZIP File Format Specification — part 4

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-003.md) · [Next](part-005.md)

<a id="source-page-1-part-4"></a>

## Source page 1 (continued, part 4)

```text
       extended information extra field.

   4.4.14 internal file attributes: (2 bytes)

       Bits 1 and 2 are reserved for use by PKWARE.

       4.4.14.1 The lowest bit of this field indicates, if set, 
       that the file is apparently an ASCII or text file.  If not
       set, that the file apparently contains binary data.
       The remaining bits are unused in version 1.0.

       4.4.14.2 The 0x0002 bit of this field indicates, if set, that 
       a 4 byte variable record length control field precedes each 
       logical record indicating the length of the record. The 
       record length control field is stored in little-endian byte
       order.  This flag is independent of text control characters, 
       and if used in conjunction with text data, includes any 
       control characters in the total length of the record. This 
       value is provided for mainframe data transfer support.

   4.4.15 external file attributes: (4 bytes)

       The mapping of the external attributes is
       host-system dependent (see 'version made by').  For
       MS-DOS, the low order byte is the MS-DOS directory
       attribute byte.  If input came from standard input, this
       field is set to zero.

   4.4.16 relative offset of local header: (4 bytes)

       This is the offset from the start of the first disk on
       which this file appears, to where the local header SHOULD
       be found.  If an archive is in ZIP64 format and the value
       in this field is 0xFFFFFFFF, the size will be in the 
       corresponding 8 byte zip64 extended information extra field.

   4.4.17 file name: (Variable)

       4.4.17.1 The name of the file, with optional relative path.
       The path stored MUST NOT contain a drive or
       device letter, or a leading slash.  All slashes
       MUST be forward slashes '/' as opposed to
       backwards slashes '\' for compatibility with Amiga
       and UNIX file systems etc.  If input came from standard
       input, there is no file name field.  

       4.4.17.2 If using the Central Directory Encryption Feature and 
       general purpose bit flag 13 is set indicating masking, the file 
       name stored in the Local Header will not be the actual file name.  
       A masking value consisting of a unique hexadecimal value will 
       be stored.  This value will be sequentially incremented for each 
       file in the archive. See the section on the Strong Encryption 
       Specification for details on retrieving the encrypted file name. 
       Refer to the section in this document entitled "Incorporating PKWARE 
       Proprietary Technology into Your Product" for more information.


   4.4.18 file comment: (Variable)

       The comment for this file.

   4.4.19 number of this disk: (2 bytes)

       The number of this disk, which contains central
       directory end record. If an archive is in ZIP64 format
       and the value in this field is 0xFFFF, the size will 
       be in the corresponding 4 byte zip64 end of central 
       directory field.


   4.4.20 number of the disk with the start of the central
            directory: (2 bytes)

       The number of the disk on which the central
       directory starts. If an archive is in ZIP64 format
       and the value in this field is 0xFFFF, the size will 
       be in the corresponding 4 byte zip64 end of central 
       directory field.

   4.4.21 total number of entries in the central dir on 
          this disk: (2 bytes)

      The number of central directory entries on this disk.
      If an archive is in ZIP64 format and the value in 
      this field is 0xFFFF, the size will be in the 
      corresponding 8 byte zip64 end of central 
      directory field.

   4.4.22 total number of entries in the central dir: (2 bytes)

      The total number of files in the .ZIP file. If an 
      archive is in ZIP64 format and the value in this field
      is 0xFFFF, the size will be in the corresponding 8 byte 
      zip64 end of central directory field.

   4.4.23 size of the central directory: (4 bytes)

      The size (in bytes) of the entire central directory.
      If an archive is in ZIP64 format and the value in 
      this field is 0xFFFFFFFF, the size will be in the 
      corresponding 8 byte zip64 end of central 
      directory field.

   4.4.24 offset of start of central directory with respect to
          the starting disk number:  (4 bytes)

      Offset of the start of the central directory on the
      disk on which the central directory starts. If an 
      archive is in ZIP64 format and the value in this 
      field is 0xFFFFFFFF, the size will be in the 
      corresponding 8 byte zip64 end of central 
      directory field.

   4.4.25 .ZIP file comment length: (2 bytes)

      The length of the comment for this .ZIP file.

   4.4.26 .ZIP file comment: (Variable)

      The comment for this .ZIP file.  ZIP file comment data
      is stored unsecured.  No encryption or data authentication
      is applied to this area at this time.  Confidential information
      SHOULD NOT be stored in this section.

   4.4.27 zip64 extensible data sector    (variable size)

      (currently reserved for use by PKWARE)


   4.4.28 extra field: (Variable)

     This SHOULD be used for storage expansion.  If additional 
     information needs to be stored within a ZIP file for special 
     application or platform needs, it SHOULD be stored here.  
     Programs supporting earlier versions of this specification can 
     then safely skip the file, and find the next file or header.  
     This field will be 0 length in version 1.0.  

     Existing extra fields are defined in the section
     Extensible data fields that follows.

4.5 Extensible data fields
--------------------------

   4.5.1 In order to allow different programs and different types
   of information to be stored in the 'extra' field in .ZIP
   files, the following structure MUST be used for all
   programs storing data in this field:

       header1+data1 + header2+data2 . . .

   Each header MUST consist of:

       Header ID - 2 bytes
       Data Size - 2 bytes

   Note: all fields stored in Intel low-byte/high-byte order.

   The Header ID field indicates the type of data that is in
   the following data block.

   Header IDs of 0 thru 31 are reserved for use by PKWARE.
   The remaining IDs can be used by third party vendors for
   proprietary usage.

   4.5.2 The current Header ID mappings defined by PKWARE are:

      0x0001        Zip64 extended information extra field
      0x0007        AV Info
      0x0008        Reserved for extended language encoding data (PFS)
                    (see APPENDIX D)
      0x0009        OS/2
      0x000a        NTFS 
      0x000c        OpenVMS
      0x000d        UNIX
      0x000e        Reserved for file stream and fork descriptors
      0x000f        Patch Descriptor
      0x0014        PKCS#7 Store for X.509 Certificates
      0x0015        X.509 Certificate ID and Signature for 
                    individual file
      0x0016        X.509 Certificate ID for Central Directory
      0x0017        Strong Encryption Header
      0x0018        Record Management Controls
      0x0019        PKCS#7 Encryption Recipient Certificate List
      0x0020        Reserved for Timestamp record
      0x0021        Policy Decryption Key Record
      0x0022        Smartcrypt Key Provider Record
      0x0023        Smartcrypt Policy Key Data Record
      0x0065        IBM S/390 (Z390), AS/400 (I400) attributes 
                    - uncompressed
      0x0066        Reserved for IBM S/390 (Z390), AS/400 (I400) 
                    attributes - compressed
      0x4690        POSZIP 4690 (reserved) 


   4.5.3 -Zip64 Extended Information Extra Field (0x0001):

      The following is the layout of the zip64 extended 
      information "extra" block. If one of the size or
      offset fields in the Local or Central directory
      record is too small to hold the required data,
      a Zip64 extended information record is created.
      The order of the fields in the zip64 extended 
      information record is fixed, but the fields MUST
      only appear if the corresponding Local or Central
      directory record field is set to 0xFFFF or 0xFFFFFFFF.

      Note: all fields stored in Intel low-byte/high-byte order.

        Value      Size       Description
        -----      ----       -----------
(ZIP64) 0x0001     2 bytes    Tag for this "extra" block type
        Size       2 bytes    Size of this "extra" block
        Original 
        Size       8 bytes    Original uncompressed file size
        Compressed
        Size       8 bytes    Size of compressed data
        Relative Header
        Offset     8 bytes    Offset of local header record
        Disk Start
        Number     4 bytes    Number of the disk on which
                              this file starts 

      This entry in the Local header MUST include BOTH original
      and compressed file size fields. If encrypting the 
      central directory and bit 13 of the general purpose bit
      flag is set indicating masking, the value stored in the
      Local Header for the original file size will be zero.


   4.5.4 -OS/2 Extra Field (0x0009):

      The following is the layout of the OS/2 attributes "extra" 
      block.  (Last Revision  09/05/95)

      Note: all fields stored in Intel low-byte/high-byte order.

        Value       Size          Description
        -----       ----          -----------
(OS/2)  0x0009      2 bytes       Tag for this "extra" block type
        TSize       2 bytes       Size for the following data block
        BSize       4 bytes       Uncompressed Block Size
        CType       2 bytes       Compression type
        EACRC       4 bytes       CRC value for uncompress block
        (var)       variable      Compressed block

      The OS/2 extended attribute structure (FEA2LIST) is 
      compressed and then stored in its entirety within this 
      structure.  There will only ever be one "block" of data in 
      VarFields[].

   4.5.5 -NTFS Extra Field (0x000a):

      The following is the layout of the NTFS attributes 
      "extra" block. (Note: At this time the Mtime, Atime
      and Ctime values MAY be used on any WIN32 system.)  

      Note: all fields stored in Intel low-byte/high-byte order.

        Value      Size       Description
        -----      ----       -----------
(NTFS)  0x000a     2 bytes    Tag for this "extra" block type
        TSize      2 bytes    Size of the total "extra" block
        Reserved   4 bytes    Reserved for future use
        Tag1       2 bytes    NTFS attribute tag value #1
        Size1      2 bytes    Size of attribute #1, in bytes
        (var)      Size1      Attribute #1 data
         .
         .
         .
         TagN       2 bytes    NTFS attribute tag value #N
         SizeN      2 bytes    Size of attribute #N, in bytes
         (var)      SizeN      Attribute #N data

       For NTFS, values for Tag1 through TagN are as follows:
       (currently only one set of attributes is defined for NTFS)

         Tag        Size       Description
         -----      ----       -----------
         0x0001     2 bytes    Tag for attribute #1 
         Size1      2 bytes    Size of attribute #1, in bytes
         Mtime      8 bytes    File last modification time
         Atime      8 bytes    File last access time
         Ctime      8 bytes    File creation time

   4.5.6 -OpenVMS Extra Field (0x000c):

       The following is the layout of the OpenVMS attributes 
       "extra" block.

       Note: all fields stored in Intel low-byte/high-byte order.

         Value      Size       Description
         -----      ----       -----------
 (VMS)   0x000c     2 bytes    Tag for this "extra" block type
         TSize      2 bytes    Size of the total "extra" block
         CRC        4 bytes    32-bit CRC for remainder of the block
         Tag1       2 bytes    OpenVMS attribute tag value #1
         Size1      2 bytes    Size of attribute #1, in bytes
         (var)      Size1      Attribute #1 data
         .
         .
         .
         TagN       2 bytes    OpenVMS attribute tag value #N
         SizeN      2 bytes    Size of attribute #N, in bytes
         (var)      SizeN      Attribute #N data

       OpenVMS Extra Field Rules:

          4.5.6.1. There will be one or more attributes present, which 
          will each be preceded by the above TagX & SizeX values.  
          These values are identical to the ATR$C_XXXX and ATR$S_XXXX 
          constants which are defined in ATR.H under OpenVMS C.  Neither 
          of these values will ever be zero.

          4.5.6.2. No word alignment or padding is performed.

          4.5.6.3. A well-behaved PKZIP/OpenVMS program SHOULD NOT produce
          more than one sub-block with the same TagX value.  Also, there MUST 
          NOT be more than one "extra" block of type 0x000c in a particular 
          directory record.

   4.5.7 -UNIX Extra Field (0x000d):

        The following is the layout of the UNIX "extra" block.
        Note: all fields are stored in Intel low-byte/high-byte 
        order.

        Value       Size          Description
        -----       ----          -----------
(UNIX)  0x000d      2 bytes       Tag for this "extra" block type
        TSize       2 bytes       Size for the following data block
        Atime       4 bytes       File last access time
        Mtime       4 bytes       File last modification time
        Uid         2 bytes       File user ID
```

