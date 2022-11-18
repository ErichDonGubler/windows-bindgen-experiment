pub type IDirectManipulationAutoScrollBehavior = *mut ::core::ffi::c_void;
pub type IDirectManipulationCompositor = *mut ::core::ffi::c_void;
pub type IDirectManipulationCompositor2 = *mut ::core::ffi::c_void;
pub type IDirectManipulationContent = *mut ::core::ffi::c_void;
pub type IDirectManipulationDeferContactService = *mut ::core::ffi::c_void;
pub type IDirectManipulationDragDropBehavior = *mut ::core::ffi::c_void;
pub type IDirectManipulationDragDropEventHandler = *mut ::core::ffi::c_void;
pub type IDirectManipulationFrameInfoProvider = *mut ::core::ffi::c_void;
pub type IDirectManipulationInteractionEventHandler = *mut ::core::ffi::c_void;
pub type IDirectManipulationManager = *mut ::core::ffi::c_void;
pub type IDirectManipulationManager2 = *mut ::core::ffi::c_void;
pub type IDirectManipulationManager3 = *mut ::core::ffi::c_void;
pub type IDirectManipulationPrimaryContent = *mut ::core::ffi::c_void;
pub type IDirectManipulationUpdateHandler = *mut ::core::ffi::c_void;
pub type IDirectManipulationUpdateManager = *mut ::core::ffi::c_void;
pub type IDirectManipulationViewport = *mut ::core::ffi::c_void;
pub type IDirectManipulationViewport2 = *mut ::core::ffi::c_void;
pub type IDirectManipulationViewportEventHandler = *mut ::core::ffi::c_void;
pub const CLSID_AutoScrollBehavior: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x26126a51_3c70_4c9a_aec2_948849eeb093);
pub const CLSID_DeferContactService: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0xd7b67cf4_84bb_434e_86ae_6592bbc9abd9);
pub const CLSID_DragDropConfigurationBehavior: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x09b01b3e_ba6c_454d_82e8_95e352329f23);
pub const CLSID_HorizontalIndicatorContent: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0xe7d18cf5_3ec7_44d5_a76b_3770f3cf903d);
pub const CLSID_VerticalIndicatorContent: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0xa10b5f17_afe0_4aa2_91e9_3e7001d2e6b4);
pub const CLSID_VirtualViewportContent: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x3206a19a_86f0_4cb4_a7f3_16e3b7e2d852);
pub const DCompManipulationCompositor: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x79dea627_a08a_43ac_8ef5_6900b9299126);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293u32;
pub const DirectManipulationManager: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x54e211b6_3650_4f75_8334_fa359598e1c5);
pub const DirectManipulationPrimaryContent: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0xcaa02661_d59e_41c7_8393_3ba3bacb6b57);
pub const DirectManipulationSharedManager: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x99793286_77cc_4b57_96db_3b354f6f9fb5);
pub const DirectManipulationUpdateManager: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x9fc1bfd5_1835_441a_b3b1_b6cc74b727d0);
pub const DirectManipulationViewport: ::windows_sys::core::GUID =
    ::windows_sys::core::GUID::from_u128(0x34e211b6_3650_4f75_8334_fa359598e1c5);
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP:
    DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD:
    DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE:
    DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION =
    32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION =
    128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL:
    DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL:
    DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY:
    DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG:
    DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG:
    DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_DRAG_DROP_STATUS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_GESTURE_CONFIGURATION = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL:
    DIRECTMANIPULATION_GESTURE_CONFIGURATION = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL:
    DIRECTMANIPULATION_GESTURE_CONFIGURATION = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_HITTEST_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_HORIZONTALALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT =
    0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT =
    1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT =
    2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT =
    4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER:
    DIRECTMANIPULATION_HORIZONTALALIGNMENT = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_INPUT_MODE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_INTERACTION_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE =
    1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE =
    2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE =
    3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE:
    DIRECTMANIPULATION_INTERACTION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM:
    DIRECTMANIPULATION_INTERACTION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = 100i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_MOTION_TYPES = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = 55i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_SNAPPOINT_COORDINATE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_SNAPPOINT_TYPE = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_STATUS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = 6i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_VERTICALALIGNMENT = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT =
    8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub type DIRECTMANIPULATION_VIEWPORT_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS =
    1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS =
    2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS =
    8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectManipulation\"`*"]
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING:
    DIRECTMANIPULATION_VIEWPORT_OPTIONS = 16i32;
