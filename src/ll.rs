// automatically generated by rust-bindgen, then editted by hand

//! # Streaming compression
//!
//! A `ZBUFFCompressionContext` object is required to track streaming operation.
//! Use `ZBUFF_createCCtx()` and `ZBUFF_freeCCtx()` to create/release resources.
//! `ZBUFFCompressionContext` objects can be reused multiple times.
//!
//! Start by initializing `ZBUFFCompressionContext`.
//! Use `ZBUFF_compressInit()` to start a new compression operation.
//! Use `ZBUFF_compressInitDictionary()` for a compression which requires a dictionary.
//!
//! Use `ZBUFF_compressContinue()` repetitively to consume input stream.
//! *srcSizePtr and *dstCapacityPtr can be any size.
//! The function will report how many bytes were read or written within *srcSizePtr and *dstCapacityPtr.
//! Note that it may not consume the entire input, in which case it's up to the caller to present again remaining data.
//! The content of @dst will be overwritten (up to *dstCapacityPtr) at each call, so save its content if it matters or change @dst .
//! @return : a hint to preferred nb of bytes to use as input for next function call (it's just a hint, to improve latency)
//!           or an error code, which can be tested using ZBUFF_isError().
//!
//! At any moment, it's possible to flush whatever data remains within buffer, using ZBUFF_compressFlush().
//! The nb of bytes written into @dst will be reported into *dstCapacityPtr.
//! Note that the function cannot output more than *dstCapacityPtr,
//! therefore, some content might still be left into internal buffer if *dstCapacityPtr is too small.
//! @return : nb of bytes still present into internal buffer (0 if it's empty)
//!           or an error code, which can be tested using ZBUFF_isError().
//!
//! ZBUFF_compressEnd() instructs to finish a frame.
//! It will perform a flush and write frame epilogue.
//! The epilogue is required for decoders to consider a frame completed.
//! Similar to ZBUFF_compressFlush(), it may not be able to output the entire internal buffer content if *dstCapacityPtr is too small.
//! In which case, call again ZBUFF_compressFlush() to complete the flush.
//! @return : nb of bytes still present into internal buffer (0 if it's empty)
//!           or an error code, which can be tested using ZBUFF_isError().
//!
//! Hint : recommended buffer sizes (not compulsory) : ZBUFF_recommendedCInSize / ZBUFF_recommendedCOutSize
//! input : ZBUFF_recommendedCInSize==128 KB block size is the internal unit, it improves latency to use this value (skipped buffering).
//! output : ZBUFF_recommendedCOutSize==ZSTD_compressBound(128 KB) + 3 + 3 : ensures it's always possible to write/flush/end a full block. Skip some buffering.
//! By using both, it ensures that input will be entirely consumed, and output will always contain the result, reducing intermediate buffering.
//!
//!
//! # Streaming decompression
//!
//! A `ZBUFFDecompressionContext` object is required to track streaming operations.
//! Use ZBUFF_createDCtx() and ZBUFF_freeDCtx() to create/release resources.
//! Use ZBUFF_decompressInit() to start a new decompression operation,
//!  or ZBUFF_decompressInitDictionary() if decompression requires a dictionary.
//! Note that `ZBUFFDecompressionContext` objects can be reused multiple times.
//!
//! Use ZBUFF_decompressContinue() repetitively to consume your input.
//! *srcSizePtr and *dstCapacityPtr can be any size.
//! The function will report how many bytes were read or written by modifying *srcSizePtr and *dstCapacityPtr.
//! Note that it may not consume the entire input, in which case it's up to the caller to present remaining input again.
//! The content of @dst will be overwritten (up to *dstCapacityPtr) at each function call, so save its content if it matters or change @dst.
//! @return : a hint to preferred nb of bytes to use as input for next function call (it's only a hint, to help latency)
//!           or 0 when a frame is completely decoded
//!           or an error code, which can be tested using ZBUFF_isError().
//!
//! Hint : recommended buffer sizes (not compulsory) : ZBUFF_recommendedDInSize() / ZBUFF_recommendedDOutSize()
//! output : ZBUFF_recommendedDOutSize==128 KB block size is the internal unit, it ensures it's always possible to write a full block when decoded.
//! input  : ZBUFF_recommendedDInSize==128Kb+3; just follow indications from ZBUFF_decompressContinue() to minimize latency. It should always be <= 128 KB + 3 .

use std::io;
use std::ffi::CStr;
use libc::{c_char, c_int, c_uint, c_void, size_t};

pub type ZBUFFCompressionContext = *mut c_void;
pub type ZBUFFDecompressionContext = *mut c_void;

pub type ZSTDCompressionContext = *mut c_void;
pub type ZSTDDecompressionContext = *mut c_void;

pub type ErrorCode = size_t;

/// Parse the result code
///
/// Returns the number of bytes written if the code represents success,
/// or the error message otherwise.
pub fn parse_code(code: ErrorCode) -> Result<usize, io::Error> {
    unsafe {
        if ZBUFF_isError(code) == 0 {
            Ok(code as usize)
        } else {
            let msg = CStr::from_ptr(ZBUFF_getErrorName(code));
            let error = io::Error::new(io::ErrorKind::Other,
                                       msg.to_str().unwrap().to_string());
            Err(error)
        }
    }
}

extern "C" {
    // zbuff.h

    pub fn ZBUFF_createCCtx() -> ZBUFFCompressionContext;
    pub fn ZBUFF_freeCCtx(cctx: ZBUFFCompressionContext) -> ErrorCode;

    pub fn ZBUFF_compressInit(cctx: ZBUFFCompressionContext,
                              compressionLevel: c_int)
                              -> ErrorCode;

    pub fn ZBUFF_compressInitDictionary(cctx: ZBUFFCompressionContext,
                                        dict: *const u8, dictSize: size_t,
                                        compressionLevel: c_int)
                                        -> ErrorCode;

    pub fn ZBUFF_compressContinue(cctx: ZBUFFCompressionContext, dst: *mut u8,
                                  dstCapacityPtr: *mut size_t, src: *const u8,
                                  srcSizePtr: *mut size_t)
                                  -> ErrorCode;
    pub fn ZBUFF_compressFlush(cctx: ZBUFFCompressionContext, dst: *mut u8,
                               dstCapacityPtr: *mut size_t)
                               -> ErrorCode;
    pub fn ZBUFF_compressEnd(cctx: ZBUFFCompressionContext, dst: *mut u8,
                             dstCapacityPtr: *mut size_t)
                             -> ErrorCode;


    // -***************************************************************************
    // ******************************************************************************
    pub fn ZBUFF_createDCtx() -> ZBUFFDecompressionContext;
    pub fn ZBUFF_freeDCtx(dctx: ZBUFFDecompressionContext) -> ErrorCode;

    pub fn ZBUFF_decompressInit(dctx: ZBUFFDecompressionContext) -> size_t;
    pub fn ZBUFF_decompressInitDictionary(dctx: ZBUFFDecompressionContext,
                                          dict: *const u8, dictSize: size_t)
                                          -> ErrorCode;
    pub fn ZBUFF_decompressContinue(dctx: ZBUFFDecompressionContext,
                                    dst: *mut u8, dstCapacityPtr: *mut size_t,
                                    src: *const u8, srcSizePtr: *mut size_t)
                                    -> ErrorCode;

    pub fn ZBUFF_isError(errorCode: size_t) -> c_uint;
    pub fn ZBUFF_getErrorName(errorCode: size_t) -> *const c_char;

    pub fn ZBUFF_recommendedCInSize() -> size_t;
    pub fn ZBUFF_recommendedCOutSize() -> size_t;

    pub fn ZBUFF_recommendedDInSize() -> size_t;
    pub fn ZBUFF_recommendedDOutSize() -> size_t;

    // zstd.h

    // Compression context memory management
    pub fn ZSTD_createCCtx() -> ZSTDCompressionContext;
    pub fn ZSTD_freeCCtx(cctx: ZSTDCompressionContext) -> ErrorCode;

    // Decompression context memory management
    pub fn ZSTD_createDCtx() -> ZSTDDecompressionContext;
    pub fn ZSTD_freeDCtx(cctx: ZSTDDecompressionContext) -> ErrorCode;

    /// Compression using a pre-defined Dictionary content (see dictBuilder).
    ///
    /// Note : dict can be NULL, in which case, it's equivalent to ZSTD_compressCCtx() */
    pub fn ZSTD_compress_usingDict(ctx: ZSTDCompressionContext, dst: *mut u8,
                                   dstCapacity: size_t, src: *const u8,
                                   srcSize: size_t, dict: *const u8,
                                   dictSize: size_t, compressionLevel: i32)
                                   -> ErrorCode;

    /// Decompression using a pre-defined Dictionary content (see dictBuilder).
    ///
    /// Dictionary must be identical to the one used during compression, otherwise regenerated data will be corrupted.
    ///
    /// Note : dict can be NULL, in which case, it's equivalent to ZSTD_decompressDCtx() */
    pub fn ZSTD_decompress_usingDict(dctx: ZSTDDecompressionContext,
                                     dst: *mut u8, dstCapacity: size_t,
                                     src: *const u8, srcSize: size_t,
                                     dict: *const u8, dictSize: size_t)
                                     -> ErrorCode;

    /// maximum compressed size (worst case scenario)
    pub fn ZSTD_compressBound(srcSize: size_t) -> size_t;

    // zdict.h

    pub fn ZDICT_trainFromBuffer(dictBuffer: *mut u8,
                                 dictBufferCapacity: size_t,
                                 samplesBuffer: *const u8,
                                 sampleSizes: *const size_t, nbSamples: size_t)
                                 -> size_t;
}
