use miniz_oxide::deflate::CompressionLevel;

const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const IEND: &[u8] = &[0, 0, 0, 0, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82];

const IHDR_CHUNK: &[u8] = b"IHDR";
const IHDR_SIZ: u32 = 13;
const EIGHT_BPP: u8 = 8;
const RGB_TYPE: u8 = 0x2;

const IDAT_CHUNK: &[u8] = b"IDAT";

const METADATA_SIZ: usize = PNG_MAGIC.len() + IEND.len() + IHDR_SIZ as usize + (4 * 6) /* chunks are 4byte (name + length + crc32), 2 chunks other than magic and IEND */ ;

fn gen_idat(img: Vec<RGB>, w: u32) -> Vec<u8> {
    let img = img
        .chunks_exact(w as usize)
        .flat_map(|row| {
            std::iter::once(0u8 /* filter */).chain(row.iter().copied().flatten())
        })
        .collect::<Vec<u8>>();
    miniz_oxide::deflate::compress_to_vec_zlib(&img, CompressionLevel::DefaultLevel.into())
}

#[allow(clippy::upper_case_acronyms)]
pub type RGB = [u8; 3];

pub struct Image(Vec<RGB>, u32, u32);

impl Image {
    pub fn new(img: Vec<RGB>, w: u32, h: u32) -> Image {
        assert!(w > 1 && h > 1, "dimensions cannot be zero sized.");
        assert_eq!(img.len(), (w * h) as usize, "image is not properly sized");
        Image(img, w, h)
    }

    pub fn into_png(self) -> Vec<u8> {
        let Image(img, width, height) = self;
        let zimg = gen_idat(img, width);
        let zimg_len = zimg.len() as u32;
        let mut buf: Vec<u8> = Vec::with_capacity(zimg.len() + METADATA_SIZ);

        buf.extend(PNG_MAGIC);

        buf.extend(IHDR_SIZ.to_be_bytes());
        // For the CRC32. The length of the field is not included.
        let off = buf.len();
        buf.extend(IHDR_CHUNK);
        buf.extend(width.to_be_bytes());
        buf.extend(height.to_be_bytes());
        buf.push(EIGHT_BPP); // bit depth
        buf.push(RGB_TYPE); // color type
        buf.extend([0, 0, 0]); // unused: commpression method, filter method and interlacing.
        let crc = crc32fast::hash(&buf[off..]);
        buf.extend(crc.to_be_bytes());

        buf.extend(zimg_len.to_be_bytes());
        let off = buf.len();
        buf.extend(IDAT_CHUNK);
        buf.extend(zimg);
        let crc = crc32fast::hash(&buf[off..]);
        buf.extend(crc.to_be_bytes());

        buf.extend(IEND);
        debug_assert!(
            buf.len() == (zimg_len as usize + METADATA_SIZ),
            "We underestimated the length of the image."
        );
        buf
    }
}
