# ZIP File Format Specification — part 9

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-008.md) · [Next](part-010.md)

<a id="source-page-1-part-9"></a>

## Source page 1 (continued, part 9)

```text
    for version I, revision 1 of the algorithm.  Storage requirements
    for using this algorithm are as follows:

    5.10.3 Parameters needed to control the algorithm are stored in the two
    bytes immediately preceding the compressed data.  These bytes are
    used to store the following fields:

    Model order - sets the maximum model order, default is 8, possible
                  values are from 2 to 16 inclusive

    Sub-allocator size - sets the size of sub-allocator in MB, default is 50,
                    possible values are from 1MB to 256MB inclusive

    Model restoration method - sets the method used to restart context
                    model at memory insufficiency, values are:

                    0 - restarts model from scratch - default
                    1 - cut off model - decreases performance by as much as 2x
                    2 - freeze context tree - not recommended

    5.10.4 An example for packing these fields into the 2 byte storage field is
    illustrated below.  These values are stored in Intel low-byte/high-byte
    order.

    wPPMd = (Model order - 1) + 
            ((Sub-allocator size - 1) << 4) + 
            (Model restoration method << 12)


5.11 AE-x Encryption marker - Method 99
-------------------------------------------

5.12 JPEG variant - Method 96
-------------------------------------------

5.13 PKWARE Data Compression Library Imploding -  Method 10
-----------------------------------------------------------

5.14 Reserved -  Method 11
-------------------------------------------

5.15 Reserved -  Method 13
-------------------------------------------

5.16 Reserved -  Method 15
-------------------------------------------

5.17 IBM z/OS CMPSC Compression - Method 16
-------------------------------------------

Method 16 utilizes the IBM hardware compression facility available
on most IBM mainframes.  Hardware compression can significantly 
increase the speed of data compression.  This method uses a variant 
of the LZ78 algorithm.  CMPSC hardware compression is performed
using the COMPRESSION CALL instruction.  

ZIP archives can be created using this method only on mainframes
supporting the CP instruction.  Extraction MAY occur on any
platform supporting this compression algorithm.  Use of this 
algorithm requires creation of a compression dictionary and
an expansion dictionary.  The expansion dictionary MUST be
placed into the ZIP archive for use on the system where
extraction will occur.

Additional information on this compression algorithm and dictionaries
can be found in the IBM provided document titled IBM ESA/390 Data 
Compression (SA22-7208-01). Storage requirements for using CMPSC 
compression are as follows.

The format for the compressed data stream placed into the ZIP
archive following the Local Header is:

    [dictionary header]
    [expansion dictionary]
    [CMPSC compressed data] 

If encryption is used to encrypt a file compressed with CMPSC, these 
sections MUST be encrypted as a single entity.

The format of the dictionary header is:

          Value            Size          Description
          -----            ----          -----------
          Version          1 byte        1
          Flags/Symsize    1 byte        Processing flags and
                                         symbol size
          DictionaryLen    4 bytes       Length of the 
                                         expansion dictionary

Explanation of processing flags and symbol size:

The high 4 bits are used to store the processing flags.  The low
4 bits represent the size of a symbol, in bits (values range
from 9-13).  Flag values are defined below.

    0x80 - expansion dictionary
    0x40 - expansion dictionary is compressed using Deflate
    0x20 - Reserved
    0x10 - Reserved


5.18 Reserved -  Method 17
-------------------------------------------

5.19 IBM TERSE -  Method 18
-------------------------------------------

5.20 IBM LZ77 z Architecture -  Method 19
-----------------------------------------

6.0  Traditional PKWARE Encryption
----------------------------------

    6.0.1 The following information discusses the decryption steps
    required to support traditional PKWARE encryption.  This
    form of encryption is considered weak by today's standards
    and its use is recommended only for situations with
    low security needs or for compatibility with older .ZIP 
    applications.

6.1 Traditional PKWARE Decryption
---------------------------------

    6.1.1 PKWARE is grateful to Mr. Roger Schlafly for his expert 
    contribution towards the development of PKWARE's traditional 
    encryption.

    6.1.2 PKZIP encrypts the compressed data stream.  Encrypted files 
    MUST be decrypted before they can be extracted to their original
    form.

    6.1.3 Each encrypted file has an extra 12 bytes stored at the start 
    of the data area defining the encryption header for that file.  The
    encryption header is originally set to random values, and then
    itself encrypted, using three, 32-bit keys.  The key values are
    initialized using the supplied encryption password.  After each byte
    is encrypted, the keys are then updated using pseudo-random number
    generation techniques in combination with the same CRC-32 algorithm
    used in PKZIP and described elsewhere in this document.

    6.1.4 The following are the basic steps required to decrypt a file:

    1) Initialize the three 32-bit keys with the password.
    2) Read and decrypt the 12-byte encryption header, further
       initializing the encryption keys.
    3) Read and decrypt the compressed data stream using the
       encryption keys.

    6.1.5 Initializing the encryption keys
        
    Key(0) <- 305419896
    Key(1) <- 591751049
    Key(2) <- 878082192

    loop for i <- 0 to length(password)-1
        update_keys(password(i))
    end loop

    Where update_keys() is defined as:

    update_keys(char):
      Key(0) <- crc32(key(0),char)
      Key(1) <- Key(1) + (Key(0) & 000000ffH)
      Key(1) <- Key(1) * 134775813 + 1
      Key(2) <- crc32(key(2),key(1) >> 24)
    end update_keys

    Where crc32(old_crc,char) is a routine that given a CRC value and a
    character, returns an updated CRC value after applying the CRC-32
    algorithm described elsewhere in this document.

    6.1.6 Decrypting the encryption header
        
    The purpose of this step is to further initialize the encryption
    keys, based on random data, to render a plaintext attack on the
    data ineffective.

    Read the 12-byte encryption header into Buffer, in locations
    Buffer(0) thru Buffer(11).

    loop for i <- 0 to 11
        C <- buffer(i) ^ decrypt_byte()
        update_keys(C)
        buffer(i) <- C
    end loop

    Where decrypt_byte() is defined as:

    unsigned char decrypt_byte()
        local unsigned short temp
        temp <- Key(2) | 2
        decrypt_byte <- (temp * (temp ^ 1)) >> 8
    end decrypt_byte

    After the header is decrypted,  the last 1 or 2 bytes in Buffer
    SHOULD be the high-order word/byte of the CRC for the file being
    decrypted, stored in Intel low-byte/high-byte order.  Versions of
    PKZIP prior to 2.0 used a 2 byte CRC check; a 1 byte CRC check is
    used on versions after 2.0.  This can be used to test if the password
    supplied is correct or not.

    6.1.7 Decrypting the compressed data stream
    
    The compressed data stream can be decrypted as follows:

    loop until done
        read a character into C
        Temp <- C ^ decrypt_byte()
        update_keys(temp)
        output Temp
    end loop


7.0 Strong Encryption Specification
-----------------------------------

   7.0.1 Portions of the Strong Encryption technology defined in this 
   specification are covered under patents and pending patent applications.
   Refer to the section in this document entitled "Incorporating 
   PKWARE Proprietary Technology into Your Product" for more information.

7.1 Strong Encryption Overview
------------------------------

   7.1.1 Version 5.x of this specification introduced support for strong 
   encryption algorithms.  These algorithms can be used with either 
   a password or an X.509v3 digital certificate to encrypt each file. 
   This format specification supports either password or certificate 
   based encryption to meet the security needs of today, to enable 
   interoperability between users within both PKI and non-PKI 
   environments, and to ensure interoperability between different 
   computing platforms that are running a ZIP program.  

   7.1.2 Password based encryption is the most common form of encryption 
   people are familiar with.  However, inherent weaknesses with 
   passwords (e.g. susceptibility to dictionary/brute force attack) 
   as well as password management and support issues make certificate 
   based encryption a more secure and scalable option.  Industry 
   efforts and support are defining and moving towards more advanced 
   security solutions built around X.509v3 digital certificates and 
   Public Key Infrastructures(PKI) because of the greater scalability, 
   administrative options, and more robust security over traditional 
   password based encryption. 

   7.1.3 Most standard encryption algorithms are supported with this
   specification. Reference implementations for many of these 
   algorithms are available from either commercial or open source 
   distributors.  Readily available cryptographic toolkits make
   implementation of the encryption features straight-forward.  
   This document is not intended to provide a treatise on data 
   encryption principles or theory.  Its purpose is to document the 
   data structures required for implementing interoperable data 
   encryption within the .ZIP format.  It is strongly recommended that 
   you have a good understanding of data encryption before reading 
   further.

   7.1.4 The algorithms introduced in Version 5.0 of this specification 
   include:

      RC2 40 bit, 64 bit, and 128 bit
      RC4 40 bit, 64 bit, and 128 bit
      DES
      3DES 112 bit and 168 bit
  
   Version 5.1 adds support for the following:

      AES 128 bit, 192 bit, and 256 bit


   7.1.5 Version 6.1 introduces encryption data changes to support 
   interoperability with Smartcard and USB Token certificate storage 
   methods which do not support the OAEP strengthening standard.

   7.1.6 Version 6.2 introduces support for encrypting metadata by compressing 
   and encrypting the central directory data structure to reduce information 
   leakage.   Information leakage can occur in legacy ZIP applications 
   through exposure of information about a file even though that file is 
   stored encrypted.  The information exposed consists of file 
   characteristics stored within the records and fields defined by this 
   specification.  This includes data such as a file's name, its original 
   size, timestamp and CRC32 value. 

   7.1.7 Version 6.3 introduces support for encrypting data using the Blowfish
   and Twofish algorithms.  These are symmetric block ciphers developed 
   by Bruce Schneier.  Blowfish supports using a variable length key from 
   32 to 448 bits.  Block size is 64 bits.  Implementations SHOULD use 16
   rounds and the only mode supported within ZIP files is CBC. Twofish 
   supports key sizes 128, 192 and 256 bits.  Block size is 128 bits.  
   Implementations SHOULD use 16 rounds and the only mode supported within
   ZIP files is CBC.  Information and source code for both Blowfish and 
   Twofish algorithms can be found on the internet.  Consult with the author
   of these algorithms for information on terms or restrictions on use.

   7.1.8 Central Directory Encryption provides greater protection against 
   information leakage by encrypting the Central Directory structure and 
   by masking key values that are replicated in the unencrypted Local 
   Header.   ZIP compatible programs that cannot interpret an encrypted 
   Central Directory structure cannot rely on the data in the corresponding 
   Local Header for decompression information.  

   7.1.9 Extra Field records that MAY contain information about a file that SHOULD 
   not be exposed SHOULD NOT be stored in the Local Header and SHOULD only 
   be written to the Central Directory where they can be encrypted.  This 
   design currently does not support streaming.  Information in the End of 
   Central Directory record, the Zip64 End of Central Directory Locator, 
   and the Zip64 End of Central Directory records are not encrypted.  Access 
   to view data on files within a ZIP file with an encrypted Central Directory
   requires the appropriate password or private key for decryption prior to 
   viewing any files, or any information about the files, in the archive.  

   7.1.10 Older ZIP compatible programs not familiar with the Central Directory 
   Encryption feature will no longer be able to recognize the Central 
   Directory and MAY assume the ZIP file is corrupt.  Programs that 
   attempt streaming access using Local Headers will see invalid 
   information for each file.  Central Directory Encryption need not be 
   used for every ZIP file.  Its use is recommended for greater security.  
   ZIP files not using Central Directory Encryption SHOULD operate as 
   in the past. 

   7.1.11 This strong encryption feature specification is intended to provide for 
   scalable, cross-platform encryption needs ranging from simple password
   encryption to authenticated public/private key encryption.  

   7.1.12 Encryption provides data confidentiality and privacy.  It is 
```

