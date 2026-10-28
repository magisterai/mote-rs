# ZIP File Format Specification — part 10

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-009.md) · [Next](part-011.md)

<a id="source-page-1-part-10"></a>

## Source page 1 (continued, part 10)

```text
   recommended that you combine X.509 digital signing with encryption 
   to add authentication and non-repudiation.


7.2 Single Password Symmetric Encryption Method
-----------------------------------------------

   7.2.1 The Single Password Symmetric Encryption Method using strong    
   encryption algorithms operates similarly to the traditional 
   PKWARE encryption defined in this format.  Additional data 
   structures are added to support the processing needs of the 
   strong algorithms.

   The Strong Encryption data structures are:

   7.2.2 General Purpose Bits - Bits 0 and 6 of the General Purpose bit 
   flag in both local and central header records.  Both bits set 
   indicates strong encryption.  Bit 13, when set indicates the Central
   Directory is encrypted and that selected fields in the Local Header
   are masked to hide their actual value.


    7.2.3 Extra Field 0x0017 in central header only.

    Fields to consider in this record are:

       7.2.3.1 Format - the data format identifier for this record.  The only
       value allowed at this time is the integer value 2.

       7.2.3.2 AlgId - integer identifier of the encryption algorithm from the
       following range

                 0x6601 - DES
                 0x6602 - RC2 (version needed to extract < 5.2)
                 0x6603 - 3DES 168
                 0x6609 - 3DES 112
                 0x660E - AES 128 
                 0x660F - AES 192 
                 0x6610 - AES 256 
                 0x6702 - RC2 (version needed to extract >= 5.2)
                 0x6720 - Blowfish
                 0x6721 - Twofish
                 0x6801 - RC4
                 0xFFFF - Unknown algorithm

       7.2.3.3 Bitlen - Explicit bit length of key

                 32 - 448 bits
           
       7.2.3.4 Flags - Processing flags needed for decryption

                 0x0001 - Password is required to decrypt
                 0x0002 - Certificates only
                 0x0003 - Password or certificate required to decrypt

                 Values > 0x0003 reserved for certificate processing


   7.2.4 Decryption header record preceding compressed file data.

                 -Decryption Header:

                  Value     Size     Description
                  -----     ----     -----------
                  IVSize    2 bytes  Size of initialization vector (IV)
                  IVData    IVSize   Initialization vector for this file
                  Size      4 bytes  Size of remaining decryption header data
                  Format    2 bytes  Format definition for this record
                  AlgID     2 bytes  Encryption algorithm identifier
                  Bitlen    2 bytes  Bit length of encryption key
                  Flags     2 bytes  Processing flags
                  ErdSize   2 bytes  Size of Encrypted Random Data
                  ErdData   ErdSize  Encrypted Random Data
                  Reserved1 4 bytes  Reserved certificate processing data
                  Reserved2 (var)    Reserved for certificate processing data
                  VSize     2 bytes  Size of password validation data
                  VData     VSize-4  Password validation data
                  VCRC32    4 bytes  Standard ZIP CRC32 of password validation data

       7.2.4.1 IVData - The size of the IV SHOULD match the algorithm block size.
       The IVData can be completely random data.  If the size of
       the randomly generated data does not match the block size
       it SHOULD be complemented with zero's or truncated as
       necessary.  If IVSize is 0,then IV = CRC32 + Uncompressed
       File Size (as a 64 bit little-endian, unsigned integer value).

       7.2.4.2 Format - the data format identifier for this record.  The only
       value allowed at this time is the integer value 3.

       7.2.4.3 AlgId - integer identifier of the encryption algorithm from the
       following range

                     0x6601 - DES
                     0x6602 - RC2 (version needed to extract < 5.2)
                     0x6603 - 3DES 168
                     0x6609 - 3DES 112
                     0x660E - AES 128 
                     0x660F - AES 192 
                     0x6610 - AES 256 
                     0x6702 - RC2 (version needed to extract >= 5.2)
                     0x6720 - Blowfish
                     0x6721 - Twofish
                     0x6801 - RC4
                     0xFFFF - Unknown algorithm

        7.2.4.4 Bitlen - Explicit bit length of key

                     32 - 448 bits
               
        7.2.4.5 Flags - Processing flags needed for decryption

                     0x0001 - Password is required to decrypt
                     0x0002 - Certificates only
                     0x0003 - Password or certificate required to decrypt

                     Values > 0x0003 reserved for certificate processing

        7.2.4.6 ErdData - Encrypted random data is used to store random data that
        is used to generate a file session key for encrypting 
        each file.  SHA1 is used to calculate hash data used to 
        derive keys.  File session keys are derived from a master 
        session key generated from the user-supplied password.
        If the Flags field in the decryption header contains 
        the value 0x4000, then the ErdData field MUST be 
        decrypted using 3DES. If the value 0x4000 is not set,
        then the ErdData field MUST be decrypted using AlgId.


        7.2.4.7 Reserved1 - Reserved for certificate processing, if value is
        zero, then Reserved2 data is absent.  See the explanation
        under the Certificate Processing Method for details on
        this data structure.

        7.2.4.8 Reserved2 - If present, the size of the Reserved2 data structure 
        is located by skipping the first 4 bytes of this field 
        and using the next 2 bytes as the remaining size.  See
        the explanation under the Certificate Processing Method
        for details on this data structure.

        7.2.4.9 VSize - This size value will always include the 4 bytes of the
        VCRC32 data and will be greater than 4 bytes.

        7.2.4.10 VData - Random data for password validation.  This data is VSize
        in length and VSize MUST be a multiple of the encryption
        block size.  VCRC32 is a checksum value of VData.  
        VData and VCRC32 are stored encrypted and start the
        stream of encrypted data for a file.


    7.2.5 Useful Tips

        7.2.5.1 Strong Encryption is always applied to a file after compression. The
        block oriented algorithms all operate in Cypher Block Chaining (CBC) 
        mode.  The block size used for AES encryption is 16.  All other block
        algorithms use a block size of 8.  Two IDs are defined for RC2 to 
        account for a discrepancy found in the implementation of the RC2
        algorithm in the cryptographic library on Windows XP SP1 and all 
        earlier versions of Windows.  It is recommended that zero length files
        not be encrypted, however programs SHOULD be prepared to extract them
        if they are found within a ZIP file.

        7.2.5.2 A pseudo-code representation of the encryption process is as follows:

            Password = GetUserPassword()
            MasterSessionKey = DeriveKey(SHA1(Password)) 
            RD = CryptographicStrengthRandomData() 
            For Each File
               IV = CryptographicStrengthRandomData() 
               VData = CryptographicStrengthRandomData()
               VCRC32 = CRC32(VData)
               FileSessionKey = DeriveKey(SHA1(IV + RD) 
               ErdData = Encrypt(RD,MasterSessionKey,IV) 
               Encrypt(VData + VCRC32 + FileData, FileSessionKey,IV)
            Done

        7.2.5.3 The function names and parameter requirements will depend on
        the choice of the cryptographic toolkit selected.  Almost any
        toolkit supporting the reference implementations for each
        algorithm can be used.  The RSA BSAFE(r), OpenSSL, and Microsoft
        CryptoAPI libraries are all known to work well.  


 7.3 Single Password - Central Directory Encryption
 --------------------------------------------------
        
    7.3.1 Central Directory Encryption is achieved within the .ZIP format by 
    encrypting the Central Directory structure.  This encapsulates the metadata 
    most often used for processing .ZIP files.  Additional metadata is stored for 
    redundancy in the Local Header for each file.  The process of concealing 
    metadata by encrypting the Central Directory does not protect the data within 
    the Local Header.  To avoid information leakage from the exposed metadata 
    in the Local Header, the fields containing information about a file are masked.  

    7.3.2 Local Header

    Masking replaces the true content of the fields for a file in the Local 
    Header with false information.  When masked, the Local Header is not 
    suitable for streaming access and the options for data recovery of damaged
    archives is reduced.  Extra Data fields that MAY contain confidential
    data SHOULD NOT be stored within the Local Header.  The value set into
    the Version needed to extract field SHOULD be the correct value needed to
    extract the file without regard to Central Directory Encryption. The fields 
    within the Local Header targeted for masking when the Central Directory is 
    encrypted are:

            Field Name                     Mask Value
            ------------------             ---------------------------
            compression method              0
            last mod file time              0
            last mod file date              0
            crc-32                          0
            compressed size                 0
            uncompressed size               0
            file name (variable size)       Base 16 value from the
                                            range 1 - 0xFFFFFFFFFFFFFFFF
                                            represented as a string whose
                                            size will be set into the
                                            file name length field

    The Base 16 value assigned as a masked file name is simply a sequentially
    incremented value for each file starting with 1 for the first file.  
    Modifications to a ZIP file MAY cause different values to be stored for 
    each file.  For compatibility, the file name field in the Local Header 
    SHOULD NOT be left blank.  As of Version 6.2 of this specification, 
    the Compression Method and Compressed Size fields are not yet masked.
    Fields having a value of 0xFFFF or 0xFFFFFFFF for the ZIP64 format
    SHOULD NOT be masked.  

    7.3.3 Encrypting the Central Directory

    Encryption of the Central Directory does not include encryption of the 
    Central Directory Signature data, the Zip64 End of Central Directory
    record, the Zip64 End of Central Directory Locator, or the End
    of Central Directory record.  The ZIP file comment data is never
    encrypted.

    Before encrypting the Central Directory, it MAY optionally be compressed.
    Compression is not required, but for storage efficiency it is assumed
    this structure will be compressed before encrypting.  Similarly, this 
    specification supports compressing the Central Directory without
    requiring that it also be encrypted.  Early implementations of this
    feature will assume the encryption method applied to files matches the 
    encryption applied to the Central Directory.

    Encryption of the Central Directory is done in a manner similar to
    that of file encryption.  The encrypted data is preceded by a 
    decryption header.  The decryption header is known as the Archive
    Decryption Header.  The fields of this record are identical to
    the decryption header preceding each encrypted file.  The location
    of the Archive Decryption Header is determined by the value in the
    Start of the Central Directory field in the Zip64 End of Central
    Directory record.  When the Central Directory is encrypted, the
    Zip64 End of Central Directory record will always be present.

    The layout of the Zip64 End of Central Directory record for all
    versions starting with 6.2 of this specification will follow the
    Version 2 format.  The Version 2 format is as follows:

    The leading fixed size fields within the Version 1 format for this
    record remain unchanged.  The record signature for both Version 1 
    and Version 2 will be 0x06064b50.  Immediately following the last
    byte of the field known as the Offset of Start of Central 
    Directory With Respect to the Starting Disk Number will begin the 
    new fields defining Version 2 of this record.  

    7.3.4 New fields for Version 2

    Note: all fields stored in Intel low-byte/high-byte order.

              Value                 Size       Description
              -----                 ----       -----------
              Compression Method    2 bytes    Method used to compress the
                                               Central Directory
              Compressed Size       8 bytes    Size of the compressed data
              Original   Size       8 bytes    Original uncompressed size
              AlgId                 2 bytes    Encryption algorithm ID
              BitLen                2 bytes    Encryption key length
              Flags                 2 bytes    Encryption flags
              HashID                2 bytes    Hash algorithm identifier
              Hash Length           2 bytes    Length of hash data
```

