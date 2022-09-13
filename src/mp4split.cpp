// https://github.com/axiomatic-systems/Bento4/blob/master/Source/C%2B%2B/Apps/Mp4Split/Mp4Split.cpp

#include <stdio.h>
#include <stdlib.h>

#include "Ap4.h"
#include "mp4split.h"
 
const unsigned int AP4_SPLIT_MAX_TRACK_IDS = 32;

/*----------------------------------------------------------------------
|   options
+---------------------------------------------------------------------*/
struct Options {
    unsigned int track_ids[AP4_SPLIT_MAX_TRACK_IDS];
    unsigned int track_id_count;
} Options;

/*----------------------------------------------------------------------
|   TrackIdMatches
+---------------------------------------------------------------------*/
static bool
TrackIdMatches(unsigned int track_id)
{
    if (Options.track_id_count == 0) return true;
    for (unsigned int i=0; i<Options.track_id_count; i++) {
        if (Options.track_ids[i] == track_id) return true;
    }
    
    return false;
}

int basic_mp4split(
    const unsigned char data[],
    unsigned int data_size,
    void* split_data,
    rust_store_callback callback
) {
    
    // default options
    Options.track_id_count = 0;
    
    AP4_MemoryByteStream* input = new AP4_MemoryByteStream(data, data_size);
    
    // get the movie
    AP4_File* file = new AP4_File(*input, true);
    AP4_Movie* movie = file->GetMovie();
    if (movie == NULL) {
        return 100;
    }
    
    // save the init segment
    AP4_MemoryByteStream* output = new AP4_MemoryByteStream();
    AP4_Result result = NULL;

    AP4_FtypAtom* ftyp = file->GetFileType(); 
    if (ftyp) {
        result = ftyp->Write(*output);
        if (AP4_FAILED(result)) {
            return 101;
        }
    }
    result = movie->GetMoovAtom()->Write(*output);
    if (AP4_FAILED(result)) {
        return 102;
    } 

    AP4_Atom* atom = NULL;
    unsigned int track_id = 0;
    AP4_DefaultAtomFactory atom_factory;
    while (true) {
        // process the next atom
        result = atom_factory.CreateAtomFromStream(*input, atom);
        if (AP4_FAILED(result)) break;
        
        if (atom->GetType() == AP4_ATOM_TYPE_MOOF) {
            AP4_ContainerAtom* moof = AP4_DYNAMIC_CAST(AP4_ContainerAtom, atom);

            unsigned int traf_count = 0;
            AP4_ContainerAtom* traf = NULL;
            do {
                traf = AP4_DYNAMIC_CAST(AP4_ContainerAtom, moof->GetChild(AP4_ATOM_TYPE_TRAF, traf_count));
                if (traf == NULL) break;
                AP4_TfhdAtom* tfhd = AP4_DYNAMIC_CAST(AP4_TfhdAtom, traf->GetChild(AP4_ATOM_TYPE_TFHD));
                if (tfhd == NULL) {
                    return 103;                    
                }
                track_id = tfhd->GetTrackId();
                traf_count++;
            } while (traf);
    
            // check if this fragment has more than one traf
            if (traf_count > 1) {
                track_id = 0;
            }
            
            // open a new file for this fragment if this moof is a segment start
            if (Options.track_id_count == 0 || track_id == Options.track_ids[0]) {
                if (output) {
                    callback(split_data, output->GetData(), output->GetDataSize());
                    output->Release();
                    output = NULL;
                }

                output = new AP4_MemoryByteStream();
            }
        }
        
        // write the atom
        if (output && atom->GetType() != AP4_ATOM_TYPE_MFRA && TrackIdMatches(track_id)) {
            atom->Write(*output);
        }

        delete atom;
    }

    callback(split_data, output->GetData(), output->GetDataSize());

    // cleanup
    delete file;
    if (input) input->Release();
    if (output) output->Release();
    
    return 0;
}
