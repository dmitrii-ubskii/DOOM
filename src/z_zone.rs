// ZONE MEMORY
// PU - purge tags.
// Tags < 100 are not overwritten until freed.
pub const PU_STATIC: usize = 1; // static entire execution time
pub const PU_SOUND: usize = 2; // static while playing
pub const PU_MUSIC: usize = 3; // static while playing
pub const PU_DAVE: usize = 4; // anything else Dave wants static
pub const PU_LEVEL: usize = 50; // static until level exited
pub const PU_LEVSPEC: usize = 51; // a special thinker in a level
// Tags >= 100 are purgable whenever needed.
pub const PU_PURGELEVEL: usize = 100;
pub const PU_CACHE: usize = 101;

/*
void	Z_Init (void);
void*	Z_Malloc (int size, int tag, void *ptr);
void    Z_Free (void *ptr);
void    Z_FreeTags (int lowtag, int hightag);
void    Z_DumpHeap (int lowtag, int hightag);
void    Z_FileDumpHeap (FILE *f);
void    Z_CheckHeap (void);
void    Z_ChangeTag2 (void *ptr, int tag);
int     Z_FreeMemory (void);


typedef struct memblock_s
{
	int			size;	// including the header and possibly tiny fragments
	void**		user;	// NULL if a free block
	int			tag;	// purgelevel
	int			id;	// should be ZONEID
	struct memblock_s*	next;
	struct memblock_s*	prev;
} memblock_t;

//
// This is used to get the local FILE:LINE info from CPP
// prior to really call the function in question.
//
#define Z_ChangeTag(p,t) \
{ \
	  if (( (memblock_t *)( (byte *)(p) - sizeof(memblock_t)))->id!=0x1d4a11) \
	  I_Error("Z_CT at "__FILE__":%i",__LINE__); \
	  Z_ChangeTag2(p,t); \
};

*/
