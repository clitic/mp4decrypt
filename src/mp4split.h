#ifndef __MP4_SPLIT_H__
#define __MP4_SPLIT_H__

#ifdef __cplusplus
extern "C"
{
#endif

    typedef void (*rust_store_callback)(void *, const unsigned char *data, unsigned int length);
    int basic_mp4split(
        const unsigned char data[],
        unsigned int data_size,
        void* split_data,
        rust_store_callback callback
    );

#ifdef __cplusplus
}
#endif

#endif /* __MP4_SPLIT_H__ */
