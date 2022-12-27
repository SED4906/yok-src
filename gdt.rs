use x86::{
    dtables::{lgdt, DescriptorTablePointer},
    segmentation::{BuildDescriptor, CodeSegmentType, DataSegmentType, Descriptor, DescriptorBuilder, SegmentDescriptorBuilder},
    Ring,
};

/// how many entries do we want in our GDT
const GDT_ENTRIES: usize = 9;

/// the GDT itself (aligned to 64 bits for performance)
static mut GDT: [Descriptor; GDT_ENTRIES] = [Descriptor::NULL; GDT_ENTRIES];

fn populate_gdt(gdt: &mut [Descriptor; GDT_ENTRIES]) {
    gdt[0] = Descriptor::NULL;
    gdt[1] = DescriptorBuilder::code_descriptor(0, 0x000fffff, CodeSegmentType::ExecuteRead)
        .present()
        .dpl(Ring::Ring0)
        .finish();
    gdt[2] = DescriptorBuilder::data_descriptor(0, 0x000fffff, DataSegmentType::ReadWrite)
        .present()
        .dpl(Ring::Ring0)
        .finish();
    gdt[3] = DescriptorBuilder::code_descriptor(0, 0x000fffff, CodeSegmentType::ExecuteRead)
        .present()
        .dpl(Ring::Ring0)
        .db()
        .limit_granularity_4kb()
        .finish();
    gdt[4] = DescriptorBuilder::data_descriptor(0, 0x000fffff, DataSegmentType::ReadWrite)
        .present()
        .dpl(Ring::Ring0)
        .db()
        .limit_granularity_4kb()
        .finish();
    gdt[5] = DescriptorBuilder::code_descriptor(0, 0, CodeSegmentType::ExecuteRead)
        .present()
        .dpl(Ring::Ring0)
        .l()
        .finish();
    gdt[6] = DescriptorBuilder::data_descriptor(0, 0, DataSegmentType::ReadWrite)
        .present()
        .dpl(Ring::Ring0)
        .finish();
    gdt[7] = DescriptorBuilder::code_descriptor(0, 0, CodeSegmentType::ExecuteRead)
        .present()
        .dpl(Ring::Ring3)
        .l()
        .finish();
    gdt[8] = DescriptorBuilder::data_descriptor(0, 0, DataSegmentType::ReadWrite)
        .present()
        .dpl(Ring::Ring3)
        .finish();
}

/// initialize GDT
pub unsafe fn init() {
    populate_gdt(&mut GDT);

    // load GDT
    lgdt(&DescriptorTablePointer::new(&GDT));
}