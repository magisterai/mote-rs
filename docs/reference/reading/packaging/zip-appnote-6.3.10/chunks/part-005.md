# ZIP File Format Specification — part 5

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-004.md) · [Next](part-006.md)

<a id="source-page-1-part-5"></a>

## Source page 1 (continued, part 5)

```text
        Gid         2 bytes       File group ID
        (var)       variable      Variable length data field

        The variable length data field will contain file type 
        specific data.  Currently the only values allowed are
        the original "linked to" file names for hard or symbolic 
        links, and the major and minor device node numbers for
        character and block device nodes.  Since device nodes
        cannot be either symbolic or hard links, only one set of
        variable length data is stored.  Link files will have the
        name of the original file stored.  This name is NOT NULL
        terminated.  Its size can be determined by checking TSize -
        12.  Device entries will have eight bytes stored as two 4
        byte entries (in little endian format).  The first entry
        will be the major device number, and the second the minor
        device number.
                          
   4.5.8 -PATCH Descriptor Extra Field (0x000f):

        4.5.8.1 The following is the layout of the Patch Descriptor 
        "extra" block.

        Note: all fields stored in Intel low-byte/high-byte order.

        Value     Size     Description
        -----     ----     -----------
(Patch) 0x000f    2 bytes  Tag for this "extra" block type
        TSize     2 bytes  Size of the total "extra" block
        Version   2 bytes  Version of the descriptor
        Flags     4 bytes  Actions and reactions (see below) 
        OldSize   4 bytes  Size of the file about to be patched 
        OldCRC    4 bytes  32-bit CRC of the file to be patched 
        NewSize   4 bytes  Size of the resulting file 
        NewCRC    4 bytes  32-bit CRC of the resulting file 

        4.5.8.2 Actions and reactions

        Bits          Description
        ----          ----------------
        0             Use for auto detection
        1             Treat as a self-patch
        2-3           RESERVED
        4-5           Action (see below)
        6-7           RESERVED
        8-9           Reaction (see below) to absent file 
        10-11         Reaction (see below) to newer file
        12-13         Reaction (see below) to unknown file
        14-15         RESERVED
        16-31         RESERVED

           4.5.8.2.1 Actions

           Action       Value
           ------       ----- 
           none         0
           add          1
           delete       2
           patch        3

           4.5.8.2.2 Reactions
        
           Reaction     Value
           --------     -----
           ask          0
           skip         1
           ignore       2
           fail         3

        4.5.8.3 Patch support is provided by PKPatchMaker(tm) technology 
        and is covered under U.S. Patents and Patents Pending. The use or 
        implementation in a product of certain technological aspects set
        forth in the current APPNOTE, including those with regard to 
        strong encryption or patching requires a license from PKWARE.  
        Refer to the section in this document entitled "Incorporating 
        PKWARE Proprietary Technology into Your Product" for more 
        information. 

   4.5.9 -PKCS#7 Store for X.509 Certificates (0x0014):

        This field MUST contain information about each of the certificates 
        files MAY be signed with. When the Central Directory Encryption 
        feature is enabled for a ZIP file, this record will appear in 
        the Archive Extra Data Record, otherwise it will appear in the 
        first central directory record and will be ignored in any 
        other record.  

                          
        Note: all fields stored in Intel low-byte/high-byte order.

        Value     Size     Description
        -----     ----     -----------
(Store) 0x0014    2 bytes  Tag for this "extra" block type
        TSize     2 bytes  Size of the store data
        TData     TSize    Data about the store


   4.5.10 -X.509 Certificate ID and Signature for individual file (0x0015):

        This field contains the information about which certificate in 
        the PKCS#7 store was used to sign a particular file. It also 
        contains the signature data. This field can appear multiple 
        times, but can only appear once per certificate.

        Note: all fields stored in Intel low-byte/high-byte order.

        Value     Size     Description
        -----     ----     -----------
(CID)   0x0015    2 bytes  Tag for this "extra" block type
        TSize     2 bytes  Size of data that follows
        TData     TSize    Signature Data

   4.5.11 -X.509 Certificate ID and Signature for central directory (0x0016):

        This field contains the information about which certificate in 
        the PKCS#7 store was used to sign the central directory structure.
        When the Central Directory Encryption feature is enabled for a 
        ZIP file, this record will appear in the Archive Extra Data Record, 
        otherwise it will appear in the first central directory record.

        Note: all fields stored in Intel low-byte/high-byte order.

        Value     Size     Description
        -----     ----     -----------
(CDID)  0x0016    2 bytes  Tag for this "extra" block type
        TSize     2 bytes  Size of data that follows
        TData     TSize    Data

   4.5.12 -Strong Encryption Header (0x0017):

        Value     Size     Description
        -----     ----     -----------
        0x0017    2 bytes  Tag for this "extra" block type
        TSize     2 bytes  Size of data that follows
        Format    2 bytes  Format definition for this record
        AlgID     2 bytes  Encryption algorithm identifier
        Bitlen    2 bytes  Bit length of encryption key
        Flags     2 bytes  Processing flags
        CertData  TSize-8  Certificate decryption extra field data
                           (refer to the explanation for CertData
                            in the section describing the 
                            Certificate Processing Method under 
                            the Strong Encryption Specification)

        See the section describing the Strong Encryption Specification 
        for details.  Refer to the section in this document entitled 
        "Incorporating PKWARE Proprietary Technology into Your Product" 
        for more information.

   4.5.13 -Record Management Controls (0x0018):

          Value     Size     Description
          -----     ----     -----------
(Rec-CTL) 0x0018    2 bytes  Tag for this "extra" block type
          CSize     2 bytes  Size of total extra block data
          Tag1      2 bytes  Record control attribute 1
          Size1     2 bytes  Size of attribute 1, in bytes
          Data1     Size1    Attribute 1 data
          .
          .
          .
          TagN      2 bytes  Record control attribute N
          SizeN     2 bytes  Size of attribute N, in bytes
          DataN     SizeN    Attribute N data


   4.5.14 -PKCS#7 Encryption Recipient Certificate List (0x0019): 

        This field MAY contain information about each of the certificates
        used in encryption processing and it can be used to identify who is
        allowed to decrypt encrypted files.  This field SHOULD only appear 
        in the archive extra data record. This field is not required and 
        serves only to aid archive modifications by preserving public 
        encryption key data. Individual security requirements may dictate 
        that this data be omitted to deter information exposure.

        Note: all fields stored in Intel low-byte/high-byte order.

         Value     Size     Description
         -----     ----     -----------
(CStore) 0x0019    2 bytes  Tag for this "extra" block type
         TSize     2 bytes  Size of the store data
         TData     TSize    Data about the store

         TData:

         Value     Size     Description
         -----     ----     -----------
         Version   2 bytes  Format version number - MUST be 0x0001 at this time
         CStore    (var)    PKCS#7 data blob

         See the section describing the Strong Encryption Specification 
         for details.  Refer to the section in this document entitled 
         "Incorporating PKWARE Proprietary Technology into Your Product" 
         for more information.

   4.5.15 -MVS Extra Field (0x0065):

        The following is the layout of the MVS "extra" block.
        Note: Some fields are stored in Big Endian format.
        All text is in EBCDIC format unless otherwise specified.
Value     Size      Description
        -----     ----      -----------
(MVS)   0x0065    2 bytes   Tag for this "extra" block type
        TSize     2 bytes   Size for the following data block
        ID        4 bytes   EBCDIC "Z390" 0xE9F3F9F0 or
                            "T4MV" for TargetFour
        (var)     TSize-4   Attribute data (see APPENDIX B)


   4.5.16 -OS/400 Extra Field (0x0065):

        The following is the layout of the OS/400 "extra" block.
        Note: Some fields are stored in Big Endian format.
        All text is in EBCDIC format unless otherwise specified.

        Value     Size       Description
        -----     ----       -----------
(OS400) 0x0065    2 bytes    Tag for this "extra" block type
        TSize     2 bytes    Size for the following data block
        ID        4 bytes    EBCDIC "I400" 0xC9F4F0F0 or
                             "T4MV" for TargetFour
        (var)     TSize-4    Attribute data (see APPENDIX A)

   4.5.17 -Policy Decryption Key Record Extra Field (0x0021):

        The following is the layout of the Policy Decryption Key "extra" block.
        TData is a variable length, variable content field.  It holds
        information about encryptions and/or encryption key sources.
        Contact PKWARE for information on current TData structures.
        Information in this "extra" block may aternatively be placed
        within comment fields.  Refer to the section in this document 
        entitled "Incorporating PKWARE Proprietary Technology into Your 
        Product" for more information.

        Value     Size       Description
        -----     ----       -----------
        0x0021    2 bytes    Tag for this "extra" block type
        TSize     2 bytes    Size for the following data block
        TData     TSize      Data about the key

   4.5.18 -Key Provider Record Extra Field (0x0022):

        The following is the layout of the Key Provider "extra" block.
        TData is a variable length, variable content field.  It holds
        information about encryptions and/or encryption key sources.
        Contact PKWARE for information on current TData structures.
        Information in this "extra" block may aternatively be placed
        within comment fields.  Refer to the section in this document 
        entitled "Incorporating PKWARE Proprietary Technology into Your 
        Product" for more information.

        Value     Size       Description
        -----     ----       -----------
        0x0022    2 bytes    Tag for this "extra" block type
        TSize     2 bytes    Size for the following data block
        TData     TSize      Data about the key

   4.5.19 -Policy Key Data Record Record Extra Field (0x0023):

        The following is the layout of the Policy Key Data "extra" block.
        TData is a variable length, variable content field.  It holds
        information about encryptions and/or encryption key sources.
        Contact PKWARE for information on current TData structures.
        Information in this "extra" block may aternatively be placed
        within comment fields.  Refer to the section in this document 
        entitled "Incorporating PKWARE Proprietary Technology into Your 
        Product" for more information.

        Value     Size       Description
        -----     ----       -----------
        0x0023    2 bytes    Tag for this "extra" block type
        TSize     2 bytes    Size for the following data block
        TData     TSize      Data about the key

4.6 Third Party Mappings
------------------------
                 
   4.6.1 Third party mappings commonly used are:

          0x07c8        Macintosh
          0x1986        Pixar USD header ID
          0x2605        ZipIt Macintosh
          0x2705        ZipIt Macintosh 1.3.5+
          0x2805        ZipIt Macintosh 1.3.5+
          0x334d        Info-ZIP Macintosh
          0x4154        Tandem
          0x4341        Acorn/SparkFS 
          0x4453        Windows NT security descriptor (binary ACL)
          0x4704        VM/CMS
          0x470f        MVS
          0x4854        THEOS (old?)
          0x4b46        FWKCS MD5 (see below)
          0x4c41        OS/2 access control list (text ACL)
          0x4d49        Info-ZIP OpenVMS 
          0x4d63        Macintosh Smartzip (??)
          0x4f4c        Xceed original location extra field
          0x5356        AOS/VS (ACL)
          0x5455        extended timestamp
          0x554e        Xceed unicode extra field
          0x5855        Info-ZIP UNIX (original, also OS/2, NT, etc)
          0x6375        Info-ZIP Unicode Comment Extra Field
          0x6542        BeOS/BeBox
          0x6854        THEOS
          0x7075        Info-ZIP Unicode Path Extra Field
          0x7441        AtheOS/Syllable
          0x756e        ASi UNIX
          0x7855        Info-ZIP UNIX (new)
          0x7875        Info-ZIP UNIX (newer UID/GID)
          0xa11e        Data Stream Alignment (Apache Commons-Compress)
          0xa220        Microsoft Open Packaging Growth Hint
          0xcafe        Java JAR file Extra Field Header ID
```

