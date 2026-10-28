# ZIP File Format Specification — part 11

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-010.md) · [Next](part-012.md)

<a id="source-page-1-part-11"></a>

## Source page 1 (continued, part 11)

```text
              Hash Data             (variable) Hash data

     The Compression Method accepts the same range of values as the 
     corresponding field in the Central Header.

     The Compressed Size and Original Size values will not include the
     data of the Central Directory Signature which is compressed or
     encrypted.

     The AlgId, BitLen, and Flags fields accept the same range of values
     the corresponding fields within the 0x0017 record. 

     Hash ID identifies the algorithm used to hash the Central Directory 
     data.  This data does not have to be hashed, in which case the
     values for both the HashID and Hash Length will be 0.  Possible 
     values for HashID are:

              Value         Algorithm
             ------         ---------
             0x0000          none
             0x0001          CRC32
             0x8003          MD5
             0x8004          SHA1
             0x8007          RIPEMD160
             0x800C          SHA256
             0x800D          SHA384
             0x800E          SHA512

     7.3.5 When the Central Directory data is signed, the same hash algorithm
     used to hash the Central Directory for signing SHOULD be used.
     This is recommended for processing efficiency, however, it is 
     permissible for any of the above algorithms to be used independent 
     of the signing process.

     The Hash Data will contain the hash data for the Central Directory.
     The length of this data will vary depending on the algorithm used.

     The Version Needed to Extract SHOULD be set to 62.

     The value for the Total Number of Entries on the Current Disk will
     be 0.  These records will no longer support random access when
     encrypting the Central Directory.

     7.3.6 When the Central Directory is compressed and/or encrypted, the
     End of Central Directory record will store the value 0xFFFFFFFF
     as the value for the Total Number of Entries in the Central
     Directory.  The value stored in the Total Number of Entries in
     the Central Directory on this Disk field will be 0.  The actual
     values will be stored in the equivalent fields of the Zip64
     End of Central Directory record.

     7.3.7 Decrypting and decompressing the Central Directory is accomplished
     in the same manner as decrypting and decompressing a file.

 7.4 Certificate Processing Method
 ---------------------------------

    The Certificate Processing Method for ZIP file encryption 
    defines the following additional data fields:

    7.4.1 Certificate Flag Values

    Additional processing flags that can be present in the Flags field of both 
    the 0x0017 field of the central directory Extra Field and the Decryption 
    header record preceding compressed file data are:

         0x0007 - reserved for future use
         0x000F - reserved for future use
         0x0100 - Indicates non-OAEP key wrapping was used.  If this
                  this field is set, the version needed to extract MUST
                  be at least 61.  This means OAEP key wrapping is not
                  used when generating a Master Session Key using
                  ErdData.
         0x4000 - ErdData MUST be decrypted using 3DES-168, otherwise use the
                  same algorithm used for encrypting the file contents.
         0x8000 - reserved for future use


    7.4.2 CertData - Extra Field 0x0017 record certificate data structure

    The data structure used to store certificate data within the section
    of the Extra Field defined by the CertData field of the 0x0017
    record are as shown:

          Value     Size     Description
          -----     ----     -----------
          RCount    4 bytes  Number of recipients.  
          HashAlg   2 bytes  Hash algorithm identifier
          HSize     2 bytes  Hash size
          SRList    (var)    Simple list of recipients hashed public keys

                          
         RCount    This defines the number intended recipients whose 
                   public keys were used for encryption.  This identifies
                   the number of elements in the SRList.

         HashAlg   This defines the hash algorithm used to calculate
                   the public key hash of each public key used
                   for encryption. This field currently supports
                   only the following value for SHA-1

                   0x8004 - SHA1

         HSize     This defines the size of a hashed public key.

         SRList    This is a variable length list of the hashed 
                   public keys for each intended recipient.  Each 
                   element in this list is HSize.  The total size of 
                   SRList is determined using RCount * HSize.


    7.4.3 Reserved1 - Certificate Decryption Header Reserved1 Data

          Value     Size     Description
          -----     ----     -----------
          RCount    4 bytes  Number of recipients.  
                      
          RCount   This defines the number intended recipients whose 
                   public keys were used for encryption.  This defines
                   the number of elements in the REList field defined below.


    7.4.4 Reserved2 - Certificate Decryption Header Reserved2 Data Structures


          Value     Size     Description
          -----     ----     -----------
          HashAlg   2 bytes  Hash algorithm identifier
          HSize     2 bytes  Hash size
          REList    (var)    List of recipient data elements


         HashAlg   This defines the hash algorithm used to calculate
                   the public key hash of each public key used
                   for encryption. This field currently supports
                   only the following value for SHA-1

                       0x8004 - SHA1

         HSize     This defines the size of a hashed public key
                   defined in REHData.

         REList    This is a variable length of list of recipient data.  
                   Each element in this list consists of a Recipient
                   Element data structure as follows:


        Recipient Element (REList) Data Structure:

              Value     Size     Description
              -----     ----     -----------
              RESize    2 bytes  Size of REHData + REKData
              REHData   HSize    Hash of recipients public key
              REKData   (var)    Simple key blob


             RESize    This defines the size of an individual REList 
                       element.  This value is the combined size of the
                       REHData field + REKData field.  REHData is defined by
                       HSize.  REKData is variable and can be calculated
                       for each REList element using RESize and HSize.

             REHData   Hashed public key for this recipient.

             REKData   Simple Key Blob.  The format of this data structure
                       is identical to that defined in the Microsoft
                       CryptoAPI and generated using the CryptExportKey()
                       function.  The version of the Simple Key Blob
                       supported at this time is 0x02 as defined by
                       Microsoft.

7.5 Certificate Processing - Central Directory Encryption
---------------------------------------------------------
        
    7.5.1 Central Directory Encryption using Digital Certificates will 
    operate in a manner similar to that of Single Password Central
    Directory Encryption.  This record will only be present when there 
    is data to place into it.  Currently, data is placed into this
    record when digital certificates are used for either encrypting 
    or signing the files within a ZIP file.  When only password 
    encryption is used with no certificate encryption or digital 
    signing, this record is not currently needed. When present, this 
    record will appear before the start of the actual Central Directory 
    data structure and will be located immediately after the Archive 
    Decryption Header if the Central Directory is encrypted.

    7.5.2 The Archive Extra Data record will be used to store the following
    information.  Additional data MAY be added in future versions.

    Extra Data Fields:

    0x0014 - PKCS#7 Store for X.509 Certificates
    0x0016 - X.509 Certificate ID and Signature for central directory
    0x0019 - PKCS#7 Encryption Recipient Certificate List

    The 0x0014 and 0x0016 Extra Data records that otherwise would be 
    located in the first record of the Central Directory for digital 
    certificate processing. When encrypting or compressing the Central 
    Directory, the 0x0014 and 0x0016 records MUST be located in the 
    Archive Extra Data record and they SHOULD NOT remain in the first 
    Central Directory record.  The Archive Extra Data record will also 
    be used to store the 0x0019 data. 

    7.5.3 When present, the size of the Archive Extra Data record will be
    included in the size of the Central Directory.  The data of the
    Archive Extra Data record will also be compressed and encrypted
    along with the Central Directory data structure.

7.6 Certificate Processing Differences
--------------------------------------

    7.6.1 The Certificate Processing Method of encryption differs from the
    Single Password Symmetric Encryption Method as follows.  Instead
    of using a user-defined password to generate a master session key,
    cryptographically random data is used.  The key material is then
    wrapped using standard key-wrapping techniques.  This key material
    is wrapped using the public key of each recipient that will need
    to decrypt the file using their corresponding private key.

    7.6.2 This specification currently assumes digital certificates will follow
    the X.509 V3 format for 1024 bit and higher RSA format digital
    certificates.  Implementation of this Certificate Processing Method
    requires supporting logic for key access and management.  This logic
    is outside the scope of this specification.

7.7 OAEP Processing with Certificate-based Encryption
-----------------------------------------------------

    7.7.1 OAEP stands for Optimal Asymmetric Encryption Padding.  It is a
    strengthening technique used for small encoded items such as decryption
    keys.  This is commonly applied in cryptographic key-wrapping techniques
    and is supported by PKCS #1.  Versions 5.0 and 6.0 of this specification 
    were designed to support OAEP key-wrapping for certificate-based 
    decryption keys for additional security.  

    7.7.2 Support for private keys stored on Smartcards or Tokens introduced
    a conflict with this OAEP logic.  Most card and token products do 
    not support the additional strengthening applied to OAEP key-wrapped 
    data.  In order to resolve this conflict, versions 6.1 and above of this 
    specification will no longer support OAEP when encrypting using 
    digital certificates. 

    7.7.3 Versions of PKZIP available during initial development of the 
    certificate processing method set a value of 61 into the 
    version needed to extract field for a file.  This indicates that 
    non-OAEP key wrapping is used.  This affects certificate encryption 
    only, and password encryption functions SHOULD NOT be affected by 
    this value.  This means values of 61 MAY be found on files encrypted
    with certificates only, or on files encrypted with both password
    encryption and certificate encryption.  Files encrypted with both
    methods can safely be decrypted using the password methods documented.

7.8 Additional Encryption/Decryption Data Records
-----------------------------------------------------

    7.8.1 Additional information MAY be stored within a ZIP file in support
    of the strong password and certificate encryption methods defined above.
    These include, but are not limited to the following record types.

      0x0021        Policy Decryption Key Record
      0x0022        Smartcrypt Key Provider Record
      0x0023        Smartcrypt Policy Key Data Record

8.0  Splitting and Spanning ZIP files
-------------------------------------

    8.1 Spanned ZIP files

      8.1.1 Spanning is the process of segmenting a ZIP file across 
      multiple removable media. This support has typically only 
      been provided for DOS formatted floppy diskettes. 

    8.2 Split ZIP files

      8.2.1 File splitting is a newer derivation of spanning.  
      Splitting follows the same segmentation process as
      spanning, however, it does not require writing each
      segment to a unique removable medium and instead supports
      placing all pieces onto local or non-removable locations
      such as file systems, local drives, folders, etc.

    8.3  File Naming Differences

      8.3.1 A key difference between spanned and split ZIP files is
      that all pieces of a spanned ZIP file have the same name.  
      Since each piece is written to a separate volume, no name 
      collisions occur and each segment can reuse the original 
      .ZIP file name given to the archive.

      8.3.2 Sequence ordering for DOS spanned archives uses the DOS 
      volume label to determine segment numbers.  Volume labels
      for each segment are written using the form PKBACK#xxx, 
      where xxx is the segment number written as a decimal 
      value from 001 - nnn.

      8.3.3 Split ZIP files are typically written to the same location
      and are subject to name collisions if the spanned name
      format is used since each segment will reside on the same 
```

