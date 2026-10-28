# ZIP File Format Specification — part 12

Source: [ZIP File Format Specification](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT) · 6.3.10, 2022-11-01.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-011.md) · [Next](part-013.md)

<a id="source-page-1-part-12"></a>

## Source page 1 (continued, part 12)

```text
      drive. To avoid name collisions, split archives are named 
      as follows.

      Segment 1   = filename.z01
      Segment n-1 = filename.z(n-1)
      Segment n   = filename.zip

      8.3.4 The .ZIP extension is used on the last segment to support
      quickly reading the central directory.  The segment number
      n SHOULD be a decimal value.
        
    8.4  Spanned Self-extracting ZIP Files
        
      8.4.1 Spanned ZIP files MAY be PKSFX Self-extracting ZIP files.
      PKSFX files MAY also be split, however, in this case
      the first segment MUST be named filename.exe.  The first
      segment of a split PKSFX archive MUST be large enough to
      include the entire executable program.

    8.5  Capacities and Markers
        
      8.5.1 Capacities for split archives are as follows:

      Maximum number of segments = 4,294,967,295 - 1
      Maximum .ZIP segment size = 4,294,967,295 bytes 
      Minimum segment size = 64K
      Maximum PKSFX segment size = 2,147,483,647 bytes
          
      8.5.2 Segment sizes MAY be different however by convention, all 
      segment sizes SHOULD be the same with the exception of the 
      last, which MAY be smaller.  Local and central directory 
      header records MUST NOT be split across a segment boundary. 
      When writing a header record, if the number of bytes remaining 
      within a segment is less than the size of the header record,
      end the current segment and write the header at the start
      of the next segment.  The central directory MAY span segment
      boundaries, but no single record in the central directory
      SHOULD be split across segments.

      8.5.3 Spanned/Split archives created using PKZIP for Windows
      (V2.50 or greater), PKZIP Command Line (V2.50 or greater),
      or PKZIP Explorer will include a special spanning 
      signature as the first 4 bytes of the first segment of
      the archive.  This signature (0x08074b50) will be 
      followed immediately by the local header signature for
      the first file in the archive.  

      8.5.4 A special spanning marker MAY also appear in spanned/split 
      archives if the spanning or splitting process starts but 
      only requires one segment.  In this case the 0x08074b50 
      signature will be replaced with the temporary spanning 
      marker signature of 0x30304b50.  Split archives can
      only be uncompressed by other versions of PKZIP that
      know how to create a split archive.

      8.5.5 The signature value 0x08074b50 is also used by some
      ZIP implementations as a marker for the Data Descriptor 
      record.  Conflict in this alternate assignment can be
      avoided by ensuring the position of the signature
      within the ZIP file to determine the use for which it
      is intended.  

9.0 Change Process
------------------

   9.1 In order for the .ZIP file format to remain a viable technology, this
   specification SHOULD be considered as open for periodic review and
   revision.  Although this format was originally designed with a 
   certain level of extensibility, not all changes in technology
   (present or future) were or will be necessarily considered in its
   design.  

   9.2 If your application requires new definitions to the
   extensible sections in this format, or if you would like to 
   submit new data structures or new capabilities, please forward 
   your request to zipformat@pkware.com.  All submissions will be 
   reviewed by the ZIP File Specification Committee for possible 
   inclusion into future versions of this specification.  

   9.3 Periodic revisions to this specification will be published as
   DRAFT or as FINAL status to ensure interoperability. We encourage 
   comments and feedback that MAY help improve clarity or content.


10.0 Incorporating PKWARE Proprietary Technology into Your Product
------------------------------------------------------------------

   10.1 The Use or Implementation in a product of APPNOTE technological 
   components pertaining to either strong encryption or patching requires 
   a separate, executed license agreement from PKWARE. Please contact 
   PKWARE at zipformat@pkware.com or +1-414-289-9788 with regard to 
   acquiring such a license.

   10.2 Additional information regarding PKWARE proprietary technology is 
   available at http://www.pkware.com/appnote.

11.0 Acknowledgements
---------------------

   In addition to the above mentioned contributors to PKZIP and PKUNZIP,
   PKWARE would like to extend special thanks to Robert Mahoney for 
   suggesting the extension .ZIP for this software.

12.0 References
---------------

   Fiala, Edward R., and Greene, Daniel H., "Data compression with
      finite windows",  Communications of the ACM, Volume 32, Number 4,
      April 1989, pages 490-505.

   Held, Gilbert, "Data Compression, Techniques and Applications,
      Hardware and Software Considerations", John Wiley & Sons, 1987.

   Huffman, D.A., "A method for the construction of minimum-redundancy
      codes", Proceedings of the IRE, Volume 40, Number 9, September 1952,
      pages 1098-1101.

   Nelson, Mark, "LZW Data Compression", Dr. Dobbs Journal, Volume 14,
      Number 10, October 1989, pages 29-37.

   Nelson, Mark, "The Data Compression Book",  M&T Books, 1991.

   Storer, James A., "Data Compression, Methods and Theory",
      Computer Science Press, 1988

   Welch, Terry, "A Technique for High-Performance Data Compression",
      IEEE Computer, Volume 17, Number 6, June 1984, pages 8-19.

   Ziv, J. and Lempel, A., "A universal algorithm for sequential data
      compression", Communications of the ACM, Volume 30, Number 6,
       June 1987, pages 520-540.

   Ziv, J. and Lempel, A., "Compression of individual sequences via
      variable-rate coding", IEEE Transactions on Information Theory,
      Volume 24, Number 5, September 1978, pages 530-536.


APPENDIX A - AS/400 Extra Field (0x0065) Attribute Definitions
--------------------------------------------------------------

A.1 Field Definition Structure:

   a. field length including length             2 bytes Big Endian
   b. field code                                2 bytes
   c. data                                      x bytes

A.2 Field Code  Description

   4001     Source type i.e. CLP etc
   4002     The text description of the library 
   4003     The text description of the file
   4004     The text description of the member
   4005     x'F0' or 0 is PF-DTA,  x'F1' or 1 is PF_SRC
   4007     Database Type Code                  1 byte
   4008     Database file and fields definition
   4009     GZIP file type                      2 bytes
   400B     IFS code page                       2 bytes
   400C     IFS Time of last file status change 4 bytes
   400D     IFS Access Time                     4 bytes
   400E     IFS Modification time               4 bytes
   005C     Length of the records in the file   2 bytes
   0068     GZIP two words                      8 bytes

APPENDIX B - z/OS Extra Field (0x0065) Attribute Definitions
------------------------------------------------------------

B.1 Field Definition Structure:

   a. field length including length             2 bytes Big Endian
   b. field code                                2 bytes
   c. data                                      x bytes

B.2 Field Code  Description

   0001     File Type                           2 bytes 
   0002     NonVSAM Record Format               1 byte
   0003     Reserved                
   0004     NonVSAM Block Size                  2 bytes Big Endian
   0005     Primary Space Allocation            3 bytes Big Endian
   0006     Secondary Space Allocation          3 bytes Big Endian
   0007     Space Allocation Type1 byte flag                
   0008     Modification Date                   Retired with PKZIP 5.0 +
   0009     Expiration Date                     Retired with PKZIP 5.0 +
   000A     PDS Directory Block Allocation      3 bytes Big Endian binary value
   000B     NonVSAM Volume List                 variable                
   000C     UNIT Reference                      Retired with PKZIP 5.0 +
   000D     DF/SMS Management Class             8 bytes EBCDIC Text Value
   000E     DF/SMS Storage Class                8 bytes EBCDIC Text Value
   000F     DF/SMS Data Class                   8 bytes EBCDIC Text Value
   0010     PDS/PDSE Member Info.               30 bytes        
   0011     VSAM sub-filetype                   2 bytes                
   0012     VSAM LRECL                          13 bytes EBCDIC "(num_avg num_max)"
   0013     VSAM Cluster Name                   Retired with PKZIP 5.0 +
   0014     VSAM KSDS Key Information           13 bytes EBCDIC "(num_length num_position)"
   0015     VSAM Average LRECL                  5 bytes EBCDIC num_value padded with blanks
   0016     VSAM Maximum LRECL                  5 bytes EBCDIC num_value padded with blanks
   0017     VSAM KSDS Key Length                5 bytes EBCDIC num_value padded with blanks
   0018     VSAM KSDS Key Position              5 bytes EBCDIC num_value padded with blanks
   0019     VSAM Data Name                      1-44 bytes EBCDIC text string
   001A     VSAM KSDS Index Name                1-44 bytes EBCDIC text string
   001B     VSAM Catalog Name                   1-44 bytes EBCDIC text string
   001C     VSAM Data Space Type                9 bytes EBCDIC text string
   001D     VSAM Data Space Primary             9 bytes EBCDIC num_value left-justified
   001E     VSAM Data Space Secondary           9 bytes EBCDIC num_value left-justified
   001F     VSAM Data Volume List               variable EBCDIC text list of 6-character Volume IDs
   0020     VSAM Data Buffer Space              8 bytes EBCDIC num_value left-justified
   0021     VSAM Data CISIZE                    5 bytes EBCDIC num_value left-justified
   0022     VSAM Erase Flag                     1 byte flag                
   0023     VSAM Free CI %                      3 bytes EBCDIC num_value left-justified
   0024     VSAM Free CA %                      3 bytes EBCDIC num_value left-justified
   0025     VSAM Index Volume List              variable EBCDIC text list of 6-character Volume IDs
   0026     VSAM Ordered Flag                   1 byte flag                
   0027     VSAM REUSE Flag                     1 byte flag                
   0028     VSAM SPANNED Flag                   1 byte flag                
   0029     VSAM Recovery Flag                  1 byte flag                
   002A     VSAM  WRITECHK  Flag                1 byte flag                
   002B     VSAM Cluster/Data SHROPTS           3 bytes EBCDIC "n,y"        
   002C     VSAM Index SHROPTS                  3 bytes EBCDIC "n,y"        
   002D     VSAM Index Space Type               9 bytes EBCDIC text string
   002E     VSAM Index Space Primary            9 bytes EBCDIC num_value left-justified
   002F     VSAM Index Space Secondary          9 bytes EBCDIC num_value left-justified
   0030     VSAM Index CISIZE                   5 bytes EBCDIC num_value left-justified
   0031     VSAM Index IMBED                    1 byte flag                
   0032     VSAM Index Ordered Flag             1 byte flag                
   0033     VSAM REPLICATE Flag                 1 byte flag                
   0034     VSAM Index REUSE Flag               1 byte flag                
   0035     VSAM Index WRITECHK Flag            1 byte flag Retired with PKZIP 5.0 +
   0036     VSAM Owner                          8 bytes EBCDIC text string
   0037     VSAM Index Owner                    8 bytes EBCDIC text string
   0038     Reserved
   0039     Reserved
   003A     Reserved
   003B     Reserved
   003C     Reserved
   003D     Reserved
   003E     Reserved
   003F     Reserved
   0040     Reserved
   0041     Reserved
   0042     Reserved
   0043     Reserved
   0044     Reserved
   0045     Reserved
   0046     Reserved
   0047     Reserved
   0048     Reserved
   0049     Reserved
   004A     Reserved
   004B     Reserved
   004C     Reserved
   004D     Reserved
   004E     Reserved
   004F     Reserved
   0050     Reserved
   0051     Reserved
   0052     Reserved
   0053     Reserved
   0054     Reserved
   0055     Reserved
   0056     Reserved
   0057     Reserved
   0058     PDS/PDSE Member TTR Info.           6 bytes  Big Endian
   0059     PDS 1st LMOD Text TTR               3 bytes  Big Endian
   005A     PDS LMOD EP Rec #                   4 bytes  Big Endian
   005B     Reserved
   005C     Max Length of records               2 bytes  Big Endian
   005D     PDSE Flag                           1 byte flag
   005E     Reserved
   005F     Reserved
   0060     Reserved
   0061     Reserved
   0062     Reserved
   0063     Reserved
   0064     Reserved
   0065     Last Date Referenced                4 bytes  Packed Hex "yyyymmdd"
   0066     Date Created                        4 bytes  Packed Hex "yyyymmdd"
   0068     GZIP two words                      8 bytes
   0071     Extended NOTE Location              12 bytes Big Endian
   0072     Archive device UNIT                 6 bytes  EBCDIC
   0073     Archive 1st Volume                  6 bytes  EBCDIC
   0074     Archive 1st VOL File Seq#           2 bytes  Binary
   0075     Native I/O Flags                    2 bytes
   0081     Unix File Type                      1 byte   enumerated
   0082     Unix File Format                    1 byte   enumerated
   0083     Unix File Character Set Tag Info    4 bytes
   0090     ZIP Environmental Processing Info   4 bytes
   0091     EAV EATTR Flags                     1 byte
```

