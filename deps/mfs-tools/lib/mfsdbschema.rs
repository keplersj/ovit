#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_db_attribute_schema_s {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_char,
    pub dependency: libc::c_char,
    pub level: libc::c_char,
    pub arity: libc::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_db_object_schema_s {
    pub name: *mut libc::c_char,
    pub nattributes: libc::c_uchar,
    pub attributes: *mut mfs_db_attribute_schema_s,
}
#[no_mangle]
pub static mut mfs_db_object_Test1_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_Test2_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_Program_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Series_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_Station_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_StationDay_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Showing_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_PlaceHolder_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DlWaiting_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LoopSet_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LoopSetClip_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SwSystem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SwModule_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Recording_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Bookmark_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Enum_attributes: [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_EnumItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Showcase_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ShowcaseItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Package_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_PackageItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Image_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_Headend_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Channel_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ResourceGroup_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ResourceItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_IndexAttr_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Preferences_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Progprefs_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_IntMatchPref_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_StringMatchPref_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Font_attributes: [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Actor_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_Outage_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ScheduledAction_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ViewerEventGroup_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ViewerEvent_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Unused_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_RecordingPart_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ObjectType_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ObjectAttribute_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SignalSource_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Setup_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_HeadendDay_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Lineup_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ComponentCode_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Component_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SeasonPass_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SoundFile_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_PostalCode_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_PrefsElement_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Person_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_Genre_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ShowingDerived_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SeriesStation_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_CityPostalCode_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_IrFormat_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_IrBlastData_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Message_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_VideoClip_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ServiceInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_IrTivoFormat_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MessageBoard_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MessageItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DataSet_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AreaCode_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_CityPhoneNum_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_User_attributes: [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SeriesCorrelation_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_CorrelationContributor_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_UserInterface_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TuikGlobal_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TuikContext_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DatabaseState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Theme_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgBoot_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgCategorySystem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgFrequency_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgNetwork_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgSatellite_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgCategoryLabel_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgProgram_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_BitrateEstimate_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MyWorldState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Test3_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgPip_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgChannel_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgChannelDefinition_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AudioBackground_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AudioBackgroundItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SubRecording_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgSchedule_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgScheduleEvent_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SatConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SatNetworkPortInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgUpdateList_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LogoGroup_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_NvRam_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_Table_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgDealerPip_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgUser_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ScartSettings_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_UpdateHistory_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AolMiniGuide_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Url_attributes: [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Asset_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_AssetHolder_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AxisSpecification_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_CaptureRequest_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ApgState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DiskPartition_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DiskConfiguration_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Clip_attributes: [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AuxInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Anchor_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_ClipPlayList_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MediaState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_UserAdvisoryRating_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AvalancheState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ModemState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MenuItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LinkTag_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LeadGeneration_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LeadGenMenuItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Registry_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RecordingBehavior_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TuikResource_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TuikResourceHolder_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TuikResourceGroup_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TuikResourceState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TuikResourceStateTemplate_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SignedFile_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AvConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_GeneralConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_PhoneConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LocationConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ArmConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_CaptureRequestBehavior_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_CorrelationPartHolder_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ServiceConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_EpgBroadcastConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_BroadcastTime_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AutoClockAdjustConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LiveCacheConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AvalancheIcebox_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AvalancheIceboxSection_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AllNightState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Stream_attributes: [mfs_db_attribute_schema_s; 0]
           =
    [];
#[no_mangle]
pub static mut mfs_db_object_LocksLimitsState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LocksLimitsContentRatingLimit_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SeasonPassCrData_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_CaptureRequestScheduleOptions_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_KnownHost_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_EncryptionKey_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RecordingQueueItem_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_NetConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_NetConfigIpParams_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_NetConfigWirelessParams_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TransferInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_UserInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DvdConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_StaticConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SpigotMap_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SpigotMapInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ModemPatch_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_VcrPlusConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TmpStorage_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TmpStoragePart_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ProgramSearchInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ModelInformation_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DvdBackground_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_FirmwareInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ClosedCaptioningState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ShowcaseAutoRecord_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SeasonPassShowcaseData_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_GroupInfo_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ProviderConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SignatureData_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ExtendedFormatString_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AdContentSet_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AdContent_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_AdContentImage_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_UnionTag_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_TagTargetAction_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_NetworkRecordRequest_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RecorderConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_FrontPanelConfig_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MessageManager_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SecureLog_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SecureLogPart_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RecordingDrm_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RecordingPartDrm_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SeasonPassTrioData_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MarshalledObject_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Candidate_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_PodChannel_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_PodChannelHolder_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RegionRatingTable_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RatingDimension_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_RatingValue_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_Cablecard_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SyncState_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LocksLimitsRegionRatingLimit_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_MenuItemFilter_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_DaylightSavingsPeriod_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_LicenseAcquisitionData_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_SeasonPassSingleExplicitData_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ServerBackup_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_object_ClipSizeKeyPair_attributes:
           [mfs_db_attribute_schema_s; 0] =
    [];
#[no_mangle]
pub static mut mfs_db_schema: [mfs_db_object_schema_s; 0] = [];
#[no_mangle]
pub static mut mfs_db_schema_nobjects: libc::c_int = 226i32;