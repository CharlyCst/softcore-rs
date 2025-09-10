#![allow(warnings)]

use softcore_prelude::*;

use crate::arch_prelude::*;

/// The software core.
///
/// This struct represents a software core, and holds all the registers as well as the core configuration.
/// The core is the main abstraction exposed by the softcore library and represents a single execution thread.
///
/// The raw functions translated directly from the specification are available in the `raw` module, whereas higher-level wrappers are implemented as methods on the [Core] struct directly.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Core {
    pub SEE: i128,
    pub FEAT_AA32EL0_IMPLEMENTED: bool,
    pub FEAT_AA32EL1_IMPLEMENTED: bool,
    pub FEAT_AA32EL2_IMPLEMENTED: bool,
    pub FEAT_AA32EL3_IMPLEMENTED: bool,
    pub FEAT_AA64EL0_IMPLEMENTED: bool,
    pub FEAT_AA64EL1_IMPLEMENTED: bool,
    pub FEAT_AA64EL2_IMPLEMENTED: bool,
    pub FEAT_AA64EL3_IMPLEMENTED: bool,
    pub FEAT_EL0_IMPLEMENTED: bool,
    pub FEAT_EL1_IMPLEMENTED: bool,
    pub FEAT_EL2_IMPLEMENTED: bool,
    pub FEAT_EL3_IMPLEMENTED: bool,
    pub FEAT_AES_IMPLEMENTED: bool,
    pub FEAT_AdvSIMD_IMPLEMENTED: bool,
    pub FEAT_CSV2_1p1_IMPLEMENTED: bool,
    pub FEAT_CSV2_1p2_IMPLEMENTED: bool,
    pub FEAT_CSV2_2_IMPLEMENTED: bool,
    pub FEAT_CSV2_3_IMPLEMENTED: bool,
    pub FEAT_DoubleLock_IMPLEMENTED: bool,
    pub FEAT_ETMv4_IMPLEMENTED: bool,
    pub FEAT_ETMv4p1_IMPLEMENTED: bool,
    pub FEAT_ETMv4p2_IMPLEMENTED: bool,
    pub FEAT_ETMv4p3_IMPLEMENTED: bool,
    pub FEAT_ETMv4p4_IMPLEMENTED: bool,
    pub FEAT_ETMv4p5_IMPLEMENTED: bool,
    pub FEAT_ETMv4p6_IMPLEMENTED: bool,
    pub FEAT_ETS2_IMPLEMENTED: bool,
    pub FEAT_FP_IMPLEMENTED: bool,
    pub FEAT_GICv3_IMPLEMENTED: bool,
    pub FEAT_GICv3_LEGACY_IMPLEMENTED: bool,
    pub FEAT_GICv3_TDIR_IMPLEMENTED: bool,
    pub FEAT_GICv3p1_IMPLEMENTED: bool,
    pub FEAT_GICv4_IMPLEMENTED: bool,
    pub FEAT_GICv4p1_IMPLEMENTED: bool,
    pub FEAT_IVIPT_IMPLEMENTED: bool,
    pub FEAT_PCSRv8_IMPLEMENTED: bool,
    pub FEAT_PMULL_IMPLEMENTED: bool,
    pub FEAT_PMUv3_IMPLEMENTED: bool,
    pub FEAT_PMUv3_EXT_IMPLEMENTED: bool,
    pub FEAT_PMUv3_EXT32_IMPLEMENTED: bool,
    pub FEAT_SHA1_IMPLEMENTED: bool,
    pub FEAT_SHA256_IMPLEMENTED: bool,
    pub FEAT_TRC_EXT_IMPLEMENTED: bool,
    pub FEAT_TRC_SR_IMPLEMENTED: bool,
    pub FEAT_nTLBPA_IMPLEMENTED: bool,
    pub FEAT_CRC32_IMPLEMENTED: bool,
    pub FEAT_Debugv8p1_IMPLEMENTED: bool,
    pub FEAT_HAFDBS_IMPLEMENTED: bool,
    pub FEAT_HPDS_IMPLEMENTED: bool,
    pub FEAT_LOR_IMPLEMENTED: bool,
    pub FEAT_LSE_IMPLEMENTED: bool,
    pub FEAT_PAN_IMPLEMENTED: bool,
    pub FEAT_PMUv3p1_IMPLEMENTED: bool,
    pub FEAT_RDM_IMPLEMENTED: bool,
    pub FEAT_VHE_IMPLEMENTED: bool,
    pub FEAT_VMID16_IMPLEMENTED: bool,
    pub FEAT_AA32BF16_IMPLEMENTED: bool,
    pub FEAT_AA32HPD_IMPLEMENTED: bool,
    pub FEAT_AA32I8MM_IMPLEMENTED: bool,
    pub FEAT_ASMv8p2_IMPLEMENTED: bool,
    pub FEAT_DPB_IMPLEMENTED: bool,
    pub FEAT_Debugv8p2_IMPLEMENTED: bool,
    pub FEAT_EDHSR_IMPLEMENTED: bool,
    pub FEAT_F32MM_IMPLEMENTED: bool,
    pub FEAT_F64MM_IMPLEMENTED: bool,
    pub FEAT_FP16_IMPLEMENTED: bool,
    pub FEAT_HPDS2_IMPLEMENTED: bool,
    pub FEAT_I8MM_IMPLEMENTED: bool,
    pub FEAT_IESB_IMPLEMENTED: bool,
    pub FEAT_LPA_IMPLEMENTED: bool,
    pub FEAT_LSMAOC_IMPLEMENTED: bool,
    pub FEAT_LVA_IMPLEMENTED: bool,
    pub FEAT_MPAM_IMPLEMENTED: bool,
    pub FEAT_PAN2_IMPLEMENTED: bool,
    pub FEAT_PCSRv8p2_IMPLEMENTED: bool,
    pub FEAT_RAS_IMPLEMENTED: bool,
    pub FEAT_SHA3_IMPLEMENTED: bool,
    pub FEAT_SHA512_IMPLEMENTED: bool,
    pub FEAT_SM3_IMPLEMENTED: bool,
    pub FEAT_SM4_IMPLEMENTED: bool,
    pub FEAT_SPE_IMPLEMENTED: bool,
    pub FEAT_SVE_IMPLEMENTED: bool,
    pub FEAT_TTCNP_IMPLEMENTED: bool,
    pub FEAT_UAO_IMPLEMENTED: bool,
    pub FEAT_VPIPT_IMPLEMENTED: bool,
    pub FEAT_XNX_IMPLEMENTED: bool,
    pub FEAT_CCIDX_IMPLEMENTED: bool,
    pub FEAT_CONSTPACFIELD_IMPLEMENTED: bool,
    pub FEAT_EPAC_IMPLEMENTED: bool,
    pub FEAT_FCMA_IMPLEMENTED: bool,
    pub FEAT_FPAC_IMPLEMENTED: bool,
    pub FEAT_FPACCOMBINE_IMPLEMENTED: bool,
    pub FEAT_JSCVT_IMPLEMENTED: bool,
    pub FEAT_LRCPC_IMPLEMENTED: bool,
    pub FEAT_NV_IMPLEMENTED: bool,
    pub FEAT_PACIMP_IMPLEMENTED: bool,
    pub FEAT_PACQARMA3_IMPLEMENTED: bool,
    pub FEAT_PACQARMA5_IMPLEMENTED: bool,
    pub FEAT_PAuth_IMPLEMENTED: bool,
    pub FEAT_SPEv1p1_IMPLEMENTED: bool,
    pub FEAT_AMUv1_IMPLEMENTED: bool,
    pub FEAT_BBM_IMPLEMENTED: bool,
    pub FEAT_CNTSC_IMPLEMENTED: bool,
    pub FEAT_DIT_IMPLEMENTED: bool,
    pub FEAT_Debugv8p4_IMPLEMENTED: bool,
    pub FEAT_DotProd_IMPLEMENTED: bool,
    pub FEAT_DoubleFault_IMPLEMENTED: bool,
    pub FEAT_FHM_IMPLEMENTED: bool,
    pub FEAT_FlagM_IMPLEMENTED: bool,
    pub FEAT_IDST_IMPLEMENTED: bool,
    pub FEAT_LRCPC2_IMPLEMENTED: bool,
    pub FEAT_LSE2_IMPLEMENTED: bool,
    pub FEAT_NV2_IMPLEMENTED: bool,
    pub FEAT_PMUv3p4_IMPLEMENTED: bool,
    pub FEAT_RASSAv1p1_IMPLEMENTED: bool,
    pub FEAT_RASv1p1_IMPLEMENTED: bool,
    pub FEAT_S2FWB_IMPLEMENTED: bool,
    pub FEAT_SEL2_IMPLEMENTED: bool,
    pub FEAT_TLBIOS_IMPLEMENTED: bool,
    pub FEAT_TLBIRANGE_IMPLEMENTED: bool,
    pub FEAT_TRF_IMPLEMENTED: bool,
    pub FEAT_TTL_IMPLEMENTED: bool,
    pub FEAT_TTST_IMPLEMENTED: bool,
    pub FEAT_BTI_IMPLEMENTED: bool,
    pub FEAT_CSV2_IMPLEMENTED: bool,
    pub FEAT_CSV3_IMPLEMENTED: bool,
    pub FEAT_DPB2_IMPLEMENTED: bool,
    pub FEAT_E0PD_IMPLEMENTED: bool,
    pub FEAT_EVT_IMPLEMENTED: bool,
    pub FEAT_ExS_IMPLEMENTED: bool,
    pub FEAT_FRINTTS_IMPLEMENTED: bool,
    pub FEAT_FlagM2_IMPLEMENTED: bool,
    pub FEAT_GTG_IMPLEMENTED: bool,
    pub FEAT_MTE_IMPLEMENTED: bool,
    pub FEAT_MTE2_IMPLEMENTED: bool,
    pub FEAT_PMUv3p5_IMPLEMENTED: bool,
    pub FEAT_RNG_IMPLEMENTED: bool,
    pub FEAT_RNG_TRAP_IMPLEMENTED: bool,
    pub FEAT_SB_IMPLEMENTED: bool,
    pub FEAT_SPECRES_IMPLEMENTED: bool,
    pub FEAT_SSBS_IMPLEMENTED: bool,
    pub FEAT_SSBS2_IMPLEMENTED: bool,
    pub FEAT_AMUv1p1_IMPLEMENTED: bool,
    pub FEAT_BF16_IMPLEMENTED: bool,
    pub FEAT_DGH_IMPLEMENTED: bool,
    pub FEAT_ECV_IMPLEMENTED: bool,
    pub FEAT_FGT_IMPLEMENTED: bool,
    pub FEAT_HPMN0_IMPLEMENTED: bool,
    pub FEAT_MPAMv0p1_IMPLEMENTED: bool,
    pub FEAT_MPAMv1p1_IMPLEMENTED: bool,
    pub FEAT_MTPMU_IMPLEMENTED: bool,
    pub FEAT_PAuth2_IMPLEMENTED: bool,
    pub FEAT_TWED_IMPLEMENTED: bool,
    pub FEAT_AFP_IMPLEMENTED: bool,
    pub FEAT_EBF16_IMPLEMENTED: bool,
    pub FEAT_HCX_IMPLEMENTED: bool,
    pub FEAT_LPA2_IMPLEMENTED: bool,
    pub FEAT_LS64_IMPLEMENTED: bool,
    pub FEAT_LS64_ACCDATA_IMPLEMENTED: bool,
    pub FEAT_LS64_V_IMPLEMENTED: bool,
    pub FEAT_MTE3_IMPLEMENTED: bool,
    pub FEAT_PAN3_IMPLEMENTED: bool,
    pub FEAT_PMUv3p7_IMPLEMENTED: bool,
    pub FEAT_RPRES_IMPLEMENTED: bool,
    pub FEAT_SPEv1p2_IMPLEMENTED: bool,
    pub FEAT_WFxT_IMPLEMENTED: bool,
    pub FEAT_XS_IMPLEMENTED: bool,
    pub FEAT_CMOW_IMPLEMENTED: bool,
    pub FEAT_Debugv8p8_IMPLEMENTED: bool,
    pub FEAT_GICv3_NMI_IMPLEMENTED: bool,
    pub FEAT_HBC_IMPLEMENTED: bool,
    pub FEAT_MOPS_IMPLEMENTED: bool,
    pub FEAT_NMI_IMPLEMENTED: bool,
    pub FEAT_PMUv3_EXT64_IMPLEMENTED: bool,
    pub FEAT_PMUv3_TH_IMPLEMENTED: bool,
    pub FEAT_PMUv3p8_IMPLEMENTED: bool,
    pub FEAT_SCTLR2_IMPLEMENTED: bool,
    pub FEAT_SPEv1p3_IMPLEMENTED: bool,
    pub FEAT_TCR2_IMPLEMENTED: bool,
    pub FEAT_TIDCP1_IMPLEMENTED: bool,
    pub FEAT_ADERR_IMPLEMENTED: bool,
    pub FEAT_AIE_IMPLEMENTED: bool,
    pub FEAT_ANERR_IMPLEMENTED: bool,
    pub FEAT_CLRBHB_IMPLEMENTED: bool,
    pub FEAT_CSSC_IMPLEMENTED: bool,
    pub FEAT_Debugv8p9_IMPLEMENTED: bool,
    pub FEAT_DoubleFault2_IMPLEMENTED: bool,
    pub FEAT_ECBHB_IMPLEMENTED: bool,
    pub FEAT_FGT2_IMPLEMENTED: bool,
    pub FEAT_HAFT_IMPLEMENTED: bool,
    pub FEAT_LRCPC3_IMPLEMENTED: bool,
    pub FEAT_MTE4_IMPLEMENTED: bool,
    pub FEAT_MTE_ASYM_FAULT_IMPLEMENTED: bool,
    pub FEAT_MTE_ASYNC_IMPLEMENTED: bool,
    pub FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED: bool,
    pub FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED: bool,
    pub FEAT_MTE_PERM_IMPLEMENTED: bool,
    pub FEAT_MTE_STORE_ONLY_IMPLEMENTED: bool,
    pub FEAT_MTE_TAGGED_FAR_IMPLEMENTED: bool,
    pub FEAT_PCSRv8p9_IMPLEMENTED: bool,
    pub FEAT_PFAR_IMPLEMENTED: bool,
    pub FEAT_PMUv3_EDGE_IMPLEMENTED: bool,
    pub FEAT_PMUv3_ICNTR_IMPLEMENTED: bool,
    pub FEAT_PMUv3_SS_IMPLEMENTED: bool,
    pub FEAT_PMUv3p9_IMPLEMENTED: bool,
    pub FEAT_PRFMSLC_IMPLEMENTED: bool,
    pub FEAT_RASSAv2_IMPLEMENTED: bool,
    pub FEAT_RASv2_IMPLEMENTED: bool,
    pub FEAT_RPRFM_IMPLEMENTED: bool,
    pub FEAT_S1PIE_IMPLEMENTED: bool,
    pub FEAT_S1POE_IMPLEMENTED: bool,
    pub FEAT_S2PIE_IMPLEMENTED: bool,
    pub FEAT_S2POE_IMPLEMENTED: bool,
    pub FEAT_SPECRES2_IMPLEMENTED: bool,
    pub FEAT_SPE_CRR_IMPLEMENTED: bool,
    pub FEAT_SPE_FDS_IMPLEMENTED: bool,
    pub FEAT_SPEv1p4_IMPLEMENTED: bool,
    pub FEAT_SPMU_IMPLEMENTED: bool,
    pub FEAT_THE_IMPLEMENTED: bool,
    pub FEAT_DoPD_IMPLEMENTED: bool,
    pub FEAT_ETE_IMPLEMENTED: bool,
    pub FEAT_SVE2_IMPLEMENTED: bool,
    pub FEAT_SVE_AES_IMPLEMENTED: bool,
    pub FEAT_SVE_BitPerm_IMPLEMENTED: bool,
    pub FEAT_SVE_PMULL128_IMPLEMENTED: bool,
    pub FEAT_SVE_SHA3_IMPLEMENTED: bool,
    pub FEAT_SVE_SM4_IMPLEMENTED: bool,
    pub FEAT_TME_IMPLEMENTED: bool,
    pub FEAT_TRBE_IMPLEMENTED: bool,
    pub FEAT_ETEv1p1_IMPLEMENTED: bool,
    pub FEAT_BRBE_IMPLEMENTED: bool,
    pub FEAT_ETEv1p2_IMPLEMENTED: bool,
    pub FEAT_RME_IMPLEMENTED: bool,
    pub FEAT_SME_IMPLEMENTED: bool,
    pub FEAT_SME_F64F64_IMPLEMENTED: bool,
    pub FEAT_SME_FA64_IMPLEMENTED: bool,
    pub FEAT_SME_I16I64_IMPLEMENTED: bool,
    pub FEAT_BRBEv1p1_IMPLEMENTED: bool,
    pub FEAT_MEC_IMPLEMENTED: bool,
    pub FEAT_SME2_IMPLEMENTED: bool,
    pub FEAT_ABLE_IMPLEMENTED: bool,
    pub FEAT_CHK_IMPLEMENTED: bool,
    pub FEAT_D128_IMPLEMENTED: bool,
    pub FEAT_EBEP_IMPLEMENTED: bool,
    pub FEAT_ETEv1p3_IMPLEMENTED: bool,
    pub FEAT_GCS_IMPLEMENTED: bool,
    pub FEAT_ITE_IMPLEMENTED: bool,
    pub FEAT_LSE128_IMPLEMENTED: bool,
    pub FEAT_LVA3_IMPLEMENTED: bool,
    pub FEAT_SEBEP_IMPLEMENTED: bool,
    pub FEAT_SME2p1_IMPLEMENTED: bool,
    pub FEAT_SME_F16F16_IMPLEMENTED: bool,
    pub FEAT_SVE2p1_IMPLEMENTED: bool,
    pub FEAT_SVE_B16B16_IMPLEMENTED: bool,
    pub FEAT_SYSINSTR128_IMPLEMENTED: bool,
    pub FEAT_SYSREG128_IMPLEMENTED: bool,
    pub FEAT_TRBE_EXT_IMPLEMENTED: bool,
    pub FEAT_TRBE_MPAM_IMPLEMENTED: bool,
    pub v8Ap0_IMPLEMENTED: bool,
    pub v8Ap1_IMPLEMENTED: bool,
    pub v8Ap2_IMPLEMENTED: bool,
    pub v8Ap3_IMPLEMENTED: bool,
    pub v8Ap4_IMPLEMENTED: bool,
    pub v8Ap5_IMPLEMENTED: bool,
    pub v8Ap6_IMPLEMENTED: bool,
    pub v8Ap7_IMPLEMENTED: bool,
    pub v8Ap8_IMPLEMENTED: bool,
    pub v8Ap9_IMPLEMENTED: bool,
    pub v9Ap0_IMPLEMENTED: bool,
    pub v9Ap1_IMPLEMENTED: bool,
    pub v9Ap2_IMPLEMENTED: bool,
    pub v9Ap3_IMPLEMENTED: bool,
    pub v9Ap4_IMPLEMENTED: bool,
    pub FeatureImpl: [bool; (259 as usize)],
    pub VariantImplemented: [bool; (16 as usize)],
    pub NUM_AMU_COUNTER_GROUPS: i128,
    pub NUM_AMU_CG0_MONITORS: i128,
    pub NUM_AMU_CG1_MONITORS: i128,
    pub NUM_PMU_COUNTERS: i128,
    pub NUM_BRBE_RECORDS: i128,
    pub NUM_GIC_PREEMPTION_BITS: i128,
    pub NUM_GIC_PRIORITY_BITS: i128,
    pub NUM_GIC_LIST_REGS: i128,
    pub NUM_BREAKPOINTS: i128,
    pub NUM_WATCHPOINTS: i128,
    pub __trcclaim_tags: BitVector<32>,
    pub __cycle_count: i128,
    pub CP15SDISABLE: Signal,
    pub CP15SDISABLE2: Signal,
    pub IsWFIsleep: bool,
    pub IsWFEsleep: bool,
    pub _ConfigReg: Configuration_Type,
    pub _DormantCtlReg: DormantCtl_Type,
    pub PMUEventAccumulator: [i128; (31 as usize)],
    pub PMULastThresholdValue: [bool; (31 as usize)],
    pub __clock_divider: i128,
    pub PSTATE: ProcState,
    pub ShouldAdvanceIT: bool,
    pub ShouldAdvanceSS: bool,
    pub EventRegister: BitVector<1>,
    pub R0: BitVector<64>,
    pub R1: BitVector<64>,
    pub R2: BitVector<64>,
    pub R3: BitVector<64>,
    pub R4: BitVector<64>,
    pub R5: BitVector<64>,
    pub R6: BitVector<64>,
    pub R7: BitVector<64>,
    pub R8: BitVector<64>,
    pub R9: BitVector<64>,
    pub R10: BitVector<64>,
    pub R11: BitVector<64>,
    pub R12: BitVector<64>,
    pub R13: BitVector<64>,
    pub R14: BitVector<64>,
    pub R15: BitVector<64>,
    pub R16: BitVector<64>,
    pub R17: BitVector<64>,
    pub R18: BitVector<64>,
    pub R19: BitVector<64>,
    pub R20: BitVector<64>,
    pub R21: BitVector<64>,
    pub R22: BitVector<64>,
    pub R23: BitVector<64>,
    pub R24: BitVector<64>,
    pub R25: BitVector<64>,
    pub R26: BitVector<64>,
    pub R27: BitVector<64>,
    pub R28: BitVector<64>,
    pub R29: BitVector<64>,
    pub R30: BitVector<64>,
    pub _PC: BitVector<64>,
    pub PhysicalCount: BitVector<88>,
    pub RC: [BitVector<64>; (5 as usize)],
    pub InGuardedPage: bool,
    pub BTypeNext: BitVector<2>,
    pub BTypeCompatible: bool,
    pub _Z: [BitVector<MAX_VL>; (32 as usize)],
    pub _P: [BitVector<MAX_PL>; (16 as usize)],
    pub _FFR: BitVector<MAX_PL>,
    pub _ZA: [BitVector<MAX_VL>; (256 as usize)],
    pub _ZT0: BitVector<512>,
    pub FPCR: FPCR_Type,
    pub FPSR: FPSR_Type,
    pub ICC_PMR_EL1: ICC_PMR_EL1_Type,
    pub TSTATE: TMState,
    pub SPESampleInFlight: bool,
    pub SPESampleContextEL1: BitVector<32>,
    pub SPESampleContextEL1Valid: bool,
    pub SPESampleContextEL2: BitVector<32>,
    pub SPESampleContextEL2Valid: bool,
    pub SPESampleInstIsNV2: bool,
    pub SPESamplePreviousBranchAddress: BitVector<64>,
    pub SPESamplePreviousBranchAddressValid: bool,
    pub SPESampleDataSource: BitVector<16>,
    pub SPESampleDataSourceValid: bool,
    pub SPESampleOpType: OpType,
    pub SPESampleClass: BitVector<2>,
    pub SPESampleSubclass: BitVector<8>,
    pub SPESampleSubclassValid: bool,
    pub SPESampleTimestamp: BitVector<64>,
    pub SPESampleTimestampValid: bool,
    pub SPESampleEvents: BitVector<64>,
    pub __SPE_LFSR: BitVector<24>,
    pub __SPE_LFSR_initialized: bool,
    pub SPERecordSize: i128,
    pub __last_cycle_count: i128,
    pub __last_branch_valid: bool,
    pub DBGEN: Signal,
    pub NIDEN: Signal,
    pub SPIDEN: Signal,
    pub SPNIDEN: Signal,
    pub RLPIDEN: Signal,
    pub RTPIDEN: Signal,
    pub __InstructionStep: bool,
    pub __CNTReadBase: BitVector<56>,
    pub __CNTBaseN: BitVector<56>,
    pub __CNTEL0BaseN: BitVector<56>,
    pub __CNTCTLBase: BitVector<56>,
    pub __RD_base: BitVector<56>,
    pub __SGI_base: BitVector<56>,
    pub __VLPI_base: BitVector<56>,
    pub __ETEBase: BitVector<56>,
    pub __ThisInstr: BitVector<32>,
    pub __ThisInstrEnc: __InstrEnc,
    pub __currentCond: BitVector<4>,
    pub Branchtypetaken: BranchType,
    pub __BranchTaken: bool,
    pub __ExclusiveMonitorSet: bool,
    pub __highest_el_aarch32: bool,
    pub __ICACHE_CCSIDR_RESET: [BitVector<64>; (7 as usize)],
    pub __DCACHE_CCSIDR_RESET: [BitVector<64>; (7 as usize)],
    pub sp_rel_access_pc: BitVector<64>,
    pub SP_mon: BitVector<32>,
    pub LR_mon: BitVector<32>,
    pub _Dclone: [BitVector<64>; (32 as usize)],
    pub HCR_EL2: HCR_EL2_Type,
    pub SCR_EL3: SCR_EL3_Type,
    pub SCR: SCR_Type,
    pub __apply_effective_shareability: bool,
    pub __cpy_mops_option_a_supported: bool,
    pub __cpyf_mops_option_a_supported: bool,
    pub __empam_force_ns_RAO: bool,
    pub __empam_force_ns_implemented: bool,
    pub __empam_sdeflt_implemented: bool,
    pub __empam_tidr_implemented: bool,
    pub __feat_rpres: bool,
    pub __has_sme_priority_control: bool,
    pub __isb_is_branch: bool,
    pub __mpam_frac: BitVector<4>,
    pub __mpam_major: BitVector<4>,
    pub __mte_implemented: BitVector<4>,
    pub __set_mops_option_a_supported: bool,
    pub __setg_mops_option_a_supported: bool,
    pub __sme_only: bool,
    pub _FPSCR: FPSCR_Type,
    pub RVBAR: BitVector<32>,
    pub ERRnFR: [ERRnFR_ElemType; (4 as usize)],
    pub CTR_EL0: CTR_EL0_Type,
    pub __block_bbm_implemented: i128,
    pub __has_sve_extended_bf16: i128,
    pub __max_implemented_smeveclen: i128,
    pub __max_implemented_sveveclen: i128,
    pub __supported_pa_size: i128,
    pub MDCR_EL2: MDCR_EL2_Type,
    pub _HDCR: HDCR_Type,
    pub MDCCSR_EL0: MDCCSR_EL0_Type,
    pub _EDSCR: EDSCR_Type,
    pub PMCNTENCLR_EL0: PMCNTENCLR_EL0_Type,
    pub _PMCNTENCLR: PMCNTENCLR_Type,
    pub PMCR_EL0: PMCR_EL0_Type,
    pub _PMCR: PMCR_Type,
    pub PMINTENCLR_EL1: PMINTENCLR_EL1_Type,
    pub _PMINTENCLR: PMINTENCLR_Type,
    pub PMOVSCLR_EL0: PMOVSCLR_EL0_Type,
    pub _PMOVSR: PMOVSR_Type,
    pub MDCR_EL3: MDCR_EL3_Type,
    pub PMCCFILTR_EL0: PMCCFILTR_EL0_Type,
    pub _PMCCFILTR: PMCCFILTR_Type,
    pub PMCNTENSET_EL0: PMCNTENSET_EL0_Type,
    pub _PMCNTENSET: PMCNTENSET_Type,
    pub PMEVTYPER_EL0: [PMEVTYPER_EL0_Type; (32 as usize)],
    pub _PMEVTYPER: [PMEVTYPER_Type; (31 as usize)],
    pub PMICFILTR_EL0: PMICFILTR_EL0_Type,
    pub SDCR: SDCR_Type,
    pub SDER32_EL2: SDER32_EL2_Type,
    pub _SDER32_EL3: SDER32_EL3_Type,
    pub _SDER: SDER_Type,
    pub PMICNTR_EL0: PMICNTR_EL0_Type,
    pub PMOVSSET_EL0: PMOVSSET_EL0_Type,
    pub BRBCR_EL1: BRBCR_EL1_Type,
    pub BRBCR_EL2: BRBCR_EL2_Type,
    pub BRBFCR_EL1: BRBFCR_EL1_Type,
    pub BRBTS_EL1: BRBTS_EL1_Type,
    pub CNTPOFF_EL2: CNTPOFF_EL2_Type,
    pub CNTVOFF_EL2: CNTVOFF_EL2_Type,
    pub CNTHCTL_EL2: CNTHCTL_EL2_Type,
    pub CFG_RVBAR: BitVector<64>,
    pub __impdef_TG0: BitVector<2>,
    pub __impdef_TG1: BitVector<2>,
    pub __mpam_has_hcr: bool,
    pub __mpam_partid_max: BitVector<16>,
    pub __mpam_pmg_max: BitVector<8>,
    pub __mpam_vpmr_max: BitVector<3>,
    pub EDECR: EDECR_Type,
    pub DLR_EL0: DLR_EL0_Type,
    pub _DLR: DLR_Type,
    pub DSPSR_EL0: DSPSR_EL0_Type,
    pub _DSPSR: DSPSR_Type,
    pub _DSPSR2: DSPSR2_Type,
    pub TCR_EL1: TCR_EL1_Type,
    pub TCR_EL2: TCR_EL2_Type,
    pub TCR_EL3: TCR_EL3_Type,
    pub BRBIDR0_EL1: BRBIDR0_EL1_Type,
    pub Records_INF: [BRBINFType; (64 as usize)],
    pub Records_SRC: [BRBSRCType; (64 as usize)],
    pub Records_TGT: [BRBTGTType; (64 as usize)],
    pub PMSIDR_EL1: PMSIDR_EL1_Type,
    pub SPESampleAddress: [BitVector<64>; (32 as usize)],
    pub SPESampleAddressValid: [bool; (32 as usize)],
    pub PMSCR_EL1: PMSCR_EL1_Type,
    pub PMSCR_EL2: PMSCR_EL2_Type,
    pub PMBLIMITR_EL1: PMBLIMITR_EL1_Type,
    pub PMBSR_EL1: PMBSR_EL1_Type,
    pub ZCR_EL1: ZCR_EL1_Type,
    pub ZCR_EL2: ZCR_EL2_Type,
    pub ZCR_EL3: ZCR_EL3_Type,
    pub SMCR_EL1: SMCR_EL1_Type,
    pub SMCR_EL2: SMCR_EL2_Type,
    pub SMCR_EL3: SMCR_EL3_Type,
    pub GCSPR_EL0: GCSPR_EL0_Type,
    pub GCSPR_EL1: GCSPR_EL1_Type,
    pub GCSPR_EL2: GCSPR_EL2_Type,
    pub GCSPR_EL3: GCSPR_EL3_Type,
    pub CPACR_EL1: CPACR_EL1_Type,
    pub CPTR_EL2: CPTR_EL2_Type,
    pub CPTR_EL3: CPTR_EL3_Type,
    pub _CPACR: CPACR_Type,
    pub _HCPTR: HCPTR_Type,
    pub NSACR: NSACR_Type,
    pub SP_EL0: SP_EL0_Type,
    pub SP_EL1: SP_EL1_Type,
    pub SP_EL2: SP_EL2_Type,
    pub SP_EL3: SP_EL3_Type,
    pub OSDLR_EL1: OSDLR_EL1_Type,
    pub _DBGOSDLR: DBGOSDLR_Type,
    pub DBGPRCR_EL1: DBGPRCR_EL1_Type,
    pub _DBGPRCR: DBGPRCR_Type,
    pub __GIC_Active: Option<InterruptID>,
    pub __GIC_Pending: Option<InterruptID>,
    pub GCSCR_EL1: GCSCR_EL1_Type,
    pub GCSCR_EL2: GCSCR_EL2_Type,
    pub GCSCR_EL3: GCSCR_EL3_Type,
    pub MDSCR_EL1: MDSCR_EL1_Type,
    pub OSLSR_EL1: OSLSR_EL1_Type,
    pub _DBGOSLSR: DBGOSLSR_Type,
    pub _HCR: HCR_Type,
    pub SCTLR_EL2: SCTLR_EL2_Type,
    pub _HSCTLR: HSCTLR_Type,
    pub SCTLR_EL1: SCTLR_EL1_Type,
    pub SCTLR_EL3: SCTLR_EL3_Type,
    pub _SCTLR_NS: SCTLR_Type,
    pub SCTLR_S: SCTLR_Type,
    pub ESR_EL1: ESR_EL1_Type,
    pub ESR_EL2: ESR_EL2_Type,
    pub ESR_EL3: ESR_EL3_Type,
    pub FAR_EL1: FAR_EL1_Type,
    pub FAR_EL2: FAR_EL2_Type,
    pub FAR_EL3: FAR_EL3_Type,
    pub HPFAR_EL2: HPFAR_EL2_Type,
    pub MFAR_EL3: MFAR_EL3_Type,
    pub PFAR_EL1: PFAR_EL1_Type,
    pub PFAR_EL2: PFAR_EL2_Type,
    pub ELR_EL1: ELR_EL1_Type,
    pub ELR_EL2: ELR_EL2_Type,
    pub ELR_EL3: ELR_EL3_Type,
    pub SPSR_EL1: SPSR_EL1_Type,
    pub SPSR_EL2: SPSR_EL2_Type,
    pub SPSR_EL3: SPSR_EL3_Type,
    pub SPSR_abt: SPSR_abt_Type,
    pub SPSR_fiq: SPSR_fiq_Type,
    pub _SPSR_hyp: SPSR_hyp_Type,
    pub SPSR_irq: SPSR_irq_Type,
    pub SPSR_mon: SPSR_mon_Type,
    pub _SPSR_svc: SPSR_svc_Type,
    pub SPSR_und: SPSR_und_Type,
    pub OSECCR_EL1: OSECCR_EL1_Type,
    pub _EDECCR: EDECCR_Type,
    pub EDESR: EDESR_Type,
    pub VBAR_EL1: VBAR_EL1_Type,
    pub VBAR_EL2: VBAR_EL2_Type,
    pub VBAR_EL3: VBAR_EL3_Type,
    pub _VBAR_NS: VBAR_Type,
    pub VBAR_S: VBAR_Type,
    pub GCSCRE0_EL1: GCSCRE0_EL1_Type,
    pub HCRX_EL2: HCRX_EL2_Type,
    pub MPAM2_EL2: MPAM2_EL2_Type,
    pub _MPAM3_EL3: MPAM3_EL3_Type,
    pub _MPAM1_EL1: MPAM1_EL1_Type,
    pub MPAMIDR_EL1: MPAMIDR_EL1_Type,
    pub MPAMHCR_EL2: MPAMHCR_EL2_Type,
    pub MPAMVPM0_EL2: MPAMVPM0_EL2_Type,
    pub MPAMVPMV_EL2: MPAMVPMV_EL2_Type,
    pub MPAMVPM1_EL2: MPAMVPM1_EL2_Type,
    pub MPAMVPM2_EL2: MPAMVPM2_EL2_Type,
    pub MPAMVPM3_EL2: MPAMVPM3_EL2_Type,
    pub MPAMVPM4_EL2: MPAMVPM4_EL2_Type,
    pub MPAMVPM5_EL2: MPAMVPM5_EL2_Type,
    pub MPAMVPM6_EL2: MPAMVPM6_EL2_Type,
    pub MPAMVPM7_EL2: MPAMVPM7_EL2_Type,
    pub MPAM0_EL1: MPAM0_EL1_Type,
    pub MPAMSM_EL1: MPAMSM_EL1_Type,
    pub CLIDR_EL1: CLIDR_EL1_Type,
    pub CONTEXTIDR_EL1: CONTEXTIDR_EL1_Type,
    pub _TTBCR_NS: TTBCR_Type,
    pub TTBCR_S: TTBCR_Type,
    pub _CONTEXTIDR_NS: CONTEXTIDR_Type,
    pub CONTEXTIDR_S: CONTEXTIDR_Type,
    pub TTBR0_NS: TTBR0_Type,
    pub TTBR0_S: TTBR0_Type,
    pub _TTBR0_EL1: TTBR0_EL1_Type,
    pub HTTBR: HTTBR_Type,
    pub _TTBR0_EL2: TTBR0_EL2_Type,
    pub TTBR1_NS: TTBR1_Type,
    pub TTBR1_S: TTBR1_Type,
    pub _TTBR1_EL1: TTBR1_EL1_Type,
    pub TTBR1_EL2: TTBR1_EL2_Type,
    pub _HDFAR: HDFAR_Type,
    pub _HIFAR: HIFAR_Type,
    pub _HPFAR: HPFAR_Type,
    pub _HSR: HSR_Type,
    pub _ELR_hyp: ELR_hyp_Type,
    pub _HVBAR: HVBAR_Type,
    pub MVBAR: MVBAR_Type,
    pub _DFAR_NS: DFAR_Type,
    pub _DFAR_S: DFAR_Type,
    pub _DFSR_NS: DFSR_Type,
    pub DFSR_S: DFSR_Type,
    pub IFSR32_EL2: IFSR32_EL2_Type,
    pub _IFSR_NS: IFSR_Type,
    pub IFSR_S: IFSR_Type,
    pub _DBGDSCRint: DBGDSCRint_Type,
    pub _DBGDSCRext: DBGDSCRext_Type,
    pub _HCR2: HCR2_Type,
    pub _IFAR_NS: IFAR_Type,
    pub _IFAR_S: IFAR_Type,
    pub SCTLR2_EL1: SCTLR2_EL1_Type,
    pub SCTLR2_EL2: SCTLR2_EL2_Type,
    pub FPEXC32_EL2: FPEXC32_EL2_Type,
    pub _FPEXC: FPEXC_Type,
    pub CNTHP_CTL_EL2: CNTHP_CTL_EL2_Type,
    pub _CNTHP_CTL: CNTHP_CTL_Type,
    pub CNTHP_CVAL_EL2: CNTHP_CVAL_EL2_Type,
    pub _CNTHP_CVAL: CNTHP_CVAL_Type,
    pub CNTP_CTL_EL0: CNTP_CTL_EL0_Type,
    pub _CNTP_CTL_NS: CNTP_CTL_Type,
    pub CNTP_CTL_S: CNTP_CTL_Type,
    pub CNTP_CVAL_EL0: CNTP_CVAL_EL0_Type,
    pub _CNTP_CVAL_NS: CNTP_CVAL_Type,
    pub CNTP_CVAL_S: CNTP_CVAL_Type,
    pub CNTV_CTL_EL0: CNTV_CTL_EL0_Type,
    pub CNTV_CVAL_EL0: CNTV_CVAL_EL0_Type,
    pub CNTHPS_CTL_EL2: CNTHPS_CTL_EL2_Type,
    pub CNTHPS_CVAL_EL2: CNTHPS_CVAL_EL2_Type,
    pub CNTHVS_CTL_EL2: CNTHVS_CTL_EL2_Type,
    pub CNTHVS_CVAL_EL2: CNTHVS_CVAL_EL2_Type,
    pub CNTHV_CTL_EL2: CNTHV_CTL_EL2_Type,
    pub CNTHV_CVAL_EL2: CNTHV_CVAL_EL2_Type,
    pub CNTPS_CTL_EL1: CNTPS_CTL_EL1_Type,
    pub CNTPS_CVAL_EL1: CNTPS_CVAL_EL1_Type,
    pub CNTCR: CNTCR_Type,
    pub CNTSCR: CNTSCR_Type,
    pub CNTKCTL_EL1: CNTKCTL_EL1_Type,
    pub GPCCR_EL3: GPCCR_EL3_Type,
    pub GPTBR_EL3: GPTBR_EL3_Type,
    pub __tlb_enabled: bool,
    pub GICC_CTLR: GICC_CTLR_Type,
    pub MAIR2_EL1: MAIR2_EL1_Type,
    pub MAIR_EL1: MAIR_EL1_Type,
    pub PIRE0_EL1: PIRE0_EL1_Type,
    pub PIR_EL1: PIR_EL1_Type,
    pub TCR2_EL1: TCR2_EL1_Type,
    pub MAIR2_EL2: MAIR2_EL2_Type,
    pub MAIR_EL2: MAIR_EL2_Type,
    pub PIR_EL2: PIR_EL2_Type,
    pub TCR2_EL2: TCR2_EL2_Type,
    pub PIRE0_EL2: PIRE0_EL2_Type,
    pub MAIR2_EL3: MAIR2_EL3_Type,
    pub MAIR_EL3: MAIR_EL3_Type,
    pub PIR_EL3: PIR_EL3_Type,
    pub SCTLR2_EL3: SCTLR2_EL3_Type,
    pub TTBR0_EL3: TTBR0_EL3_Type,
    pub APIAKeyHi_EL1: APIAKeyHi_EL1_Type,
    pub APIAKeyLo_EL1: APIAKeyLo_EL1_Type,
    pub APIBKeyHi_EL1: APIBKeyHi_EL1_Type,
    pub APIBKeyLo_EL1: APIBKeyLo_EL1_Type,
    pub APDAKeyHi_EL1: APDAKeyHi_EL1_Type,
    pub APDAKeyLo_EL1: APDAKeyLo_EL1_Type,
    pub APDBKeyHi_EL1: APDBKeyHi_EL1_Type,
    pub APDBKeyLo_EL1: APDBKeyLo_EL1_Type,
    pub APGAKeyHi_EL1: APGAKeyHi_EL1_Type,
    pub APGAKeyLo_EL1: APGAKeyLo_EL1_Type,
    pub _CNTKCTL: CNTKCTL_Type,
    pub GCR_EL1: GCR_EL1_Type,
    pub RGSR_EL1: RGSR_EL1_Type,
    pub TFSRE0_EL1: TFSRE0_EL1_Type,
    pub TFSR_EL1: TFSR_EL1_Type,
    pub TFSR_EL2: TFSR_EL2_Type,
    pub TFSR_EL3: TFSR_EL3_Type,
    pub CONTEXTIDR_EL2: CONTEXTIDR_EL2_Type,
    pub DBGBCR_EL1: [DBGBCR_EL1_Type; (64 as usize)],
    pub DBGBVR_EL1: [DBGBVR_EL1_Type; (64 as usize)],
    pub _EDSCR2: EDSCR2_Type,
    pub VTCR_EL2: VTCR_EL2_Type,
    pub VTTBR: VTTBR_Type,
    pub _VTTBR_EL2: VTTBR_EL2_Type,
    pub DBGWCR_EL1: [DBGWCR_EL1_Type; (64 as usize)],
    pub DBGWVR_EL1: [DBGWVR_EL1_Type; (64 as usize)],
    pub EDWAR: EDWAR_Type,
    pub POR_EL1: POR_EL1_Type,
    pub POR_EL2: POR_EL2_Type,
    pub POR_EL3: POR_EL3_Type,
    pub POR_EL0: POR_EL0_Type,
    pub MECID_P0_EL2: MECID_P0_EL2_Type,
    pub VMECID_P_EL2: VMECID_P_EL2_Type,
    pub MECID_A0_EL2: MECID_A0_EL2_Type,
    pub MECID_A1_EL2: MECID_A1_EL2_Type,
    pub MECID_P1_EL2: MECID_P1_EL2_Type,
    pub MECID_RL_A_EL3: MECID_RL_A_EL3_Type,
    pub S2PIR_EL2: S2PIR_EL2_Type,
    pub VSTCR_EL2: VSTCR_EL2_Type,
    pub VSTTBR_EL2: VSTTBR_EL2_Type,
    pub S2POR_EL1: S2POR_EL1_Type,
    pub VMECID_A_EL2: VMECID_A_EL2_Type,
    pub SPESampleCounterPending: [bool; (32 as usize)],
    pub SPESampleCounterValid: [bool; (32 as usize)],
    pub RCWMASK_EL1: RCWMASK_EL1_Type,
    pub RCWSMASK_EL1: RCWSMASK_EL1_Type,
    pub VNCR_EL2: VNCR_EL2_Type,
    pub HFGITR_EL2: HFGITR_EL2_Type,
    pub DISR_EL1: DISR_EL1_Type,
    pub _DISR: DISR_Type,
    pub VSESR_EL2: VSESR_EL2_Type,
    pub _VDFSR: VDFSR_Type,
    pub VDISR_EL2: VDISR_EL2_Type,
    pub _VDISR: VDISR_Type,
    pub RVBAR_EL1: RVBAR_EL1_Type,
    pub RVBAR_EL2: RVBAR_EL2_Type,
    pub RVBAR_EL3: RVBAR_EL3_Type,
    pub AMCIDR0: AMCIDR0_Type,
    pub AMCIDR1: AMCIDR1_Type,
    pub AMCIDR2: AMCIDR2_Type,
    pub AMCIDR3: AMCIDR3_Type,
    pub AMDEVTYPE: AMDEVTYPE_Type,
    pub AMPIDR0: AMPIDR0_Type,
    pub AMPIDR1: AMPIDR1_Type,
    pub AMPIDR2: AMPIDR2_Type,
    pub AMPIDR3: AMPIDR3_Type,
    pub AMPIDR4: AMPIDR4_Type,
    pub CNTEL0ACR: CNTEL0ACR_Type,
    pub CNTID: CNTID_Type,
    pub CNTNSAR: CNTNSAR_Type,
    pub CNTSR: CNTSR_Type,
    pub CTIAUTHSTATUS: CTIAUTHSTATUS_Type,
    pub CTICIDR0: CTICIDR0_Type,
    pub CTICIDR1: CTICIDR1_Type,
    pub CTICIDR2: CTICIDR2_Type,
    pub CTICIDR3: CTICIDR3_Type,
    pub CTICONTROL: CTICONTROL_Type,
    pub CTIDEVCTL: CTIDEVCTL_Type,
    pub CTIDEVID: CTIDEVID_Type,
    pub CTIDEVID1: CTIDEVID1_Type,
    pub CTIDEVID2: CTIDEVID2_Type,
    pub CTIDEVTYPE: CTIDEVTYPE_Type,
    pub CTIITCTRL: CTIITCTRL_Type,
    pub CTILAR: CTILAR_Type,
    pub CTILSR: CTILSR_Type,
    pub CTIPIDR0: CTIPIDR0_Type,
    pub CTIPIDR1: CTIPIDR1_Type,
    pub CTIPIDR2: CTIPIDR2_Type,
    pub CTIPIDR3: CTIPIDR3_Type,
    pub CTIPIDR4: CTIPIDR4_Type,
    pub DBGDEVID1: DBGDEVID1_Type,
    pub DBGDEVID2: DBGDEVID2_Type,
    pub DBGDIDR: DBGDIDR_Type,
    pub DBGDSAR: DBGDSAR_Type,
    pub DBGWFAR: DBGWFAR_Type,
    pub EDAA32PFR: EDAA32PFR_Type,
    pub EDCIDR0: EDCIDR0_Type,
    pub EDCIDR1: EDCIDR1_Type,
    pub EDCIDR2: EDCIDR2_Type,
    pub EDCIDR3: EDCIDR3_Type,
    pub EDDEVID: EDDEVID_Type,
    pub EDDEVID1: EDDEVID1_Type,
    pub EDDEVID2: EDDEVID2_Type,
    pub EDDEVTYPE: EDDEVTYPE_Type,
    pub EDDFR: EDDFR_Type,
    pub EDDFR1: EDDFR1_Type,
    pub EDHSR: EDHSR_Type,
    pub EDITCTRL: EDITCTRL_Type,
    pub EDLAR: EDLAR_Type,
    pub EDLSR: EDLSR_Type,
    pub EDPCSR: EDPCSR_Type,
    pub EDPFR: EDPFR_Type,
    pub EDPIDR0: EDPIDR0_Type,
    pub EDPIDR1: EDPIDR1_Type,
    pub EDPIDR2: EDPIDR2_Type,
    pub EDPIDR3: EDPIDR3_Type,
    pub EDPIDR4: EDPIDR4_Type,
    pub EDPRCR: EDPRCR_Type,
    pub EDPRSR: EDPRSR_Type,
    pub EDRCR: EDRCR_Type,
    pub EDVIDSR: EDVIDSR_Type,
    pub FCSEIDR: FCSEIDR_Type,
    pub GICC_ABPR: GICC_ABPR_Type,
    pub GICC_AEOIR: GICC_AEOIR_Type,
    pub GICC_AHPPIR: GICC_AHPPIR_Type,
    pub GICC_AIAR: GICC_AIAR_Type,
    pub GICC_BPR: GICC_BPR_Type,
    pub GICC_DIR: GICC_DIR_Type,
    pub GICC_EOIR: GICC_EOIR_Type,
    pub GICC_HPPIR: GICC_HPPIR_Type,
    pub GICC_IAR: GICC_IAR_Type,
    pub GICC_PMR: GICC_PMR_Type,
    pub GICC_RPR: GICC_RPR_Type,
    pub GICC_STATUSR: GICC_STATUSR_Type,
    pub GICD_CLRSPI_NSR: GICD_CLRSPI_NSR_Type,
    pub GICD_CLRSPI_SR: GICD_CLRSPI_SR_Type,
    pub GICD_CTLR: GICD_CTLR_Type,
    pub GICD_IIDR: GICD_IIDR_Type,
    pub GICD_SETSPI_NSR: GICD_SETSPI_NSR_Type,
    pub GICD_SETSPI_SR: GICD_SETSPI_SR_Type,
    pub GICD_SGIR: GICD_SGIR_Type,
    pub GICD_STATUSR: GICD_STATUSR_Type,
    pub GICD_TYPER2: GICD_TYPER2_Type,
    pub GICH_EISR: GICH_EISR_Type,
    pub GICH_ELRSR: GICH_ELRSR_Type,
    pub GICH_HCR: GICH_HCR_Type,
    pub GICH_MISR: GICH_MISR_Type,
    pub GICH_VMCR: GICH_VMCR_Type,
    pub GICH_VTR: GICH_VTR_Type,
    pub GICM_CLRSPI_NSR: GICM_CLRSPI_NSR_Type,
    pub GICM_CLRSPI_SR: GICM_CLRSPI_SR_Type,
    pub GICM_IIDR: GICM_IIDR_Type,
    pub GICM_SETSPI_NSR: GICM_SETSPI_NSR_Type,
    pub GICM_SETSPI_SR: GICM_SETSPI_SR_Type,
    pub GICM_TYPER: GICM_TYPER_Type,
    pub GICR_CLRLPIR: GICR_CLRLPIR_Type,
    pub GICR_CTLR: GICR_CTLR_Type,
    pub GICR_IIDR: GICR_IIDR_Type,
    pub GICR_INMIR0: GICR_INMIR0_Type,
    pub GICR_INVALLR: GICR_INVALLR_Type,
    pub GICR_INVLPIR: GICR_INVLPIR_Type,
    pub GICR_ISENABLER0: GICR_ISENABLER0_Type,
    pub GICR_MPAMIDR: GICR_MPAMIDR_Type,
    pub GICR_PARTIDR: GICR_PARTIDR_Type,
    pub GICR_PENDBASER: GICR_PENDBASER_Type,
    pub GICR_PROPBASER: GICR_PROPBASER_Type,
    pub GICR_SETLPIR: GICR_SETLPIR_Type,
    pub GICR_STATUSR: GICR_STATUSR_Type,
    pub GICR_SYNCR: GICR_SYNCR_Type,
    pub GICR_VPENDBASER: GICR_VPENDBASER_Type,
    pub GICR_VPROPBASER: GICR_VPROPBASER_Type,
    pub GICR_VSGIPENDR: GICR_VSGIPENDR_Type,
    pub GICR_VSGIR: GICR_VSGIR_Type,
    pub GICR_WAKER: GICR_WAKER_Type,
    pub GICV_ABPR: GICV_ABPR_Type,
    pub GICV_AEOIR: GICV_AEOIR_Type,
    pub GICV_AHPPIR: GICV_AHPPIR_Type,
    pub GICV_AIAR: GICV_AIAR_Type,
    pub GICV_BPR: GICV_BPR_Type,
    pub GICV_CTLR: GICV_CTLR_Type,
    pub GICV_DIR: GICV_DIR_Type,
    pub GICV_EOIR: GICV_EOIR_Type,
    pub GICV_HPPIR: GICV_HPPIR_Type,
    pub GICV_IAR: GICV_IAR_Type,
    pub GICV_PMR: GICV_PMR_Type,
    pub GICV_RPR: GICV_RPR_Type,
    pub GICV_STATUSR: GICV_STATUSR_Type,
    pub GITS_CBASER: GITS_CBASER_Type,
    pub GITS_CREADR: GITS_CREADR_Type,
    pub GITS_CTLR: GITS_CTLR_Type,
    pub GITS_CWRITER: GITS_CWRITER_Type,
    pub GITS_IIDR: GITS_IIDR_Type,
    pub GITS_MPAMIDR: GITS_MPAMIDR_Type,
    pub GITS_MPIDR: GITS_MPIDR_Type,
    pub GITS_PARTIDR: GITS_PARTIDR_Type,
    pub GITS_SGIR: GITS_SGIR_Type,
    pub GITS_STATUSR: GITS_STATUSR_Type,
    pub GITS_TYPER: GITS_TYPER_Type,
    pub ICC_MCTLR: ICC_MCTLR_Type,
    pub ICC_MGRPEN1: ICC_MGRPEN1_Type,
    pub ICC_MSRE: ICC_MSRE_Type,
    pub JIDR: JIDR_Type,
    pub JMCR: JMCR_Type,
    pub JOSCR: JOSCR_Type,
    pub PMAUTHSTATUS: PMAUTHSTATUS_Type,
    pub PMCFGR: PMCFGR_Type,
    pub PMCGCR0: PMCGCR0_Type,
    pub PMCIDR0: PMCIDR0_Type,
    pub PMCIDR1: PMCIDR1_Type,
    pub PMCIDR2: PMCIDR2_Type,
    pub PMCIDR3: PMCIDR3_Type,
    pub PMDEVID: PMDEVID_Type,
    pub PMDEVTYPE: PMDEVTYPE_Type,
    pub PMIIDR: PMIIDR_Type,
    pub PMITCTRL: PMITCTRL_Type,
    pub PMLAR: PMLAR_Type,
    pub PMLSR: PMLSR_Type,
    pub PMMIR: PMMIR_Type,
    pub PMPCSCTL: PMPCSCTL_Type,
    pub PMPCSR: PMPCSR_Type,
    pub PMPIDR0: PMPIDR0_Type,
    pub PMPIDR1: PMPIDR1_Type,
    pub PMPIDR2: PMPIDR2_Type,
    pub PMPIDR3: PMPIDR3_Type,
    pub PMPIDR4: PMPIDR4_Type,
    pub PMVCIDSR: PMVCIDSR_Type,
    pub PMVIDSR: PMVIDSR_Type,
    pub AMCFGR_EL0: AMCFGR_EL0_Type,
    pub _AMCFGR: AMCFGR_Type,
    pub AMCGCR_EL0: AMCGCR_EL0_Type,
    pub _AMCGCR: AMCGCR_Type,
    pub AMCNTENCLR0_EL0: AMCNTENCLR0_EL0_Type,
    pub _AMCNTENCLR0: AMCNTENCLR0_Type,
    pub AMCNTENCLR1_EL0: AMCNTENCLR1_EL0_Type,
    pub _AMCNTENCLR1: AMCNTENCLR1_Type,
    pub AMCNTENSET0_EL0: AMCNTENSET0_EL0_Type,
    pub _AMCNTENSET0: AMCNTENSET0_Type,
    pub AMCNTENSET1_EL0: AMCNTENSET1_EL0_Type,
    pub _AMCNTENSET1: AMCNTENSET1_Type,
    pub AMCR_EL0: AMCR_EL0_Type,
    pub _AMCR: AMCR_Type,
    pub AMUSERENR_EL0: AMUSERENR_EL0_Type,
    pub _AMUSERENR: AMUSERENR_Type,
    pub CCSIDR2_EL1: CCSIDR2_EL1_Type,
    pub _CCSIDR2: CCSIDR2_Type,
    pub CCSIDR_EL1: CCSIDR_EL1_Type,
    pub _CCSIDR: CCSIDR_Type,
    pub _CNTHCTL: CNTHCTL_Type,
    pub _CNTHPS_CTL: CNTHPS_CTL_Type,
    pub _CNTHVS_CTL: CNTHVS_CTL_Type,
    pub _CNTHV_CTL: CNTHV_CTL_Type,
    pub _CNTV_CTL: CNTV_CTL_Type,
    pub CSSELR_S: CSSELR_Type,
    pub CSSELR_EL1: CSSELR_EL1_Type,
    pub _CSSELR_NS: CSSELR_Type,
    pub _CTR: CTR_Type,
    pub DBGAUTHSTATUS_EL1: DBGAUTHSTATUS_EL1_Type,
    pub _DBGAUTHSTATUS: DBGAUTHSTATUS_Type,
    pub DBGCLAIMCLR_EL1: DBGCLAIMCLR_EL1_Type,
    pub _DBGCLAIMCLR: DBGCLAIMCLR_Type,
    pub DBGCLAIMSET_EL1: DBGCLAIMSET_EL1_Type,
    pub _DBGCLAIMSET: DBGCLAIMSET_Type,
    pub MDCCINT_EL1: MDCCINT_EL1_Type,
    pub _DBGDCCINT: DBGDCCINT_Type,
    pub MDRAR_EL1: MDRAR_EL1_Type,
    pub _DBGDRAR: DBGDRAR_Type,
    pub DBGVCR32_EL2: DBGVCR32_EL2_Type,
    pub _DBGVCR: DBGVCR_Type,
    pub ERRIDR_EL1: ERRIDR_EL1_Type,
    pub _ERRIDR: ERRIDR_Type,
    pub ERRSELR_EL1: ERRSELR_EL1_Type,
    pub _ERRSELR: ERRSELR_Type,
    pub RMR_EL2: RMR_EL2_Type,
    pub _HRMR: HRMR_Type,
    pub HSTR_EL2: HSTR_EL2_Type,
    pub _HSTR: HSTR_Type,
    pub _HTCR: HTCR_Type,
    pub TRFCR_EL2: TRFCR_EL2_Type,
    pub _HTRFCR: HTRFCR_Type,
    pub ICC_ASGI1R_EL1: ICC_ASGI1R_EL1_Type,
    pub _ICC_ASGI1R: ICC_ASGI1R_Type,
    pub ICC_BPR0_EL1: ICC_BPR0_EL1_Type,
    pub _ICC_BPR0: ICC_BPR0_Type,
    pub ICC_BPR1_EL1_NS: ICC_BPR1_EL1_Type,
    pub _ICC_BPR1_NS: ICC_BPR1_Type,
    pub ICC_BPR1_EL1_S: ICC_BPR1_EL1_Type,
    pub _ICC_BPR1_S: ICC_BPR1_Type,
    pub ICC_CTLR_EL1_NS: ICC_CTLR_EL1_Type,
    pub _ICC_CTLR_NS: ICC_CTLR_Type,
    pub ICC_CTLR_EL1_S: ICC_CTLR_EL1_Type,
    pub _ICC_CTLR_S: ICC_CTLR_Type,
    pub ICC_DIR_EL1: ICC_DIR_EL1_Type,
    pub _ICC_DIR: ICC_DIR_Type,
    pub ICC_EOIR0_EL1: ICC_EOIR0_EL1_Type,
    pub _ICC_EOIR0: ICC_EOIR0_Type,
    pub ICC_EOIR1_EL1: ICC_EOIR1_EL1_Type,
    pub _ICC_EOIR1: ICC_EOIR1_Type,
    pub ICC_HPPIR0_EL1: ICC_HPPIR0_EL1_Type,
    pub _ICC_HPPIR0: ICC_HPPIR0_Type,
    pub ICC_HPPIR1_EL1: ICC_HPPIR1_EL1_Type,
    pub _ICC_HPPIR1: ICC_HPPIR1_Type,
    pub ICC_SRE_EL2: ICC_SRE_EL2_Type,
    pub _ICC_HSRE: ICC_HSRE_Type,
    pub ICC_IAR0_EL1: ICC_IAR0_EL1_Type,
    pub _ICC_IAR0: ICC_IAR0_Type,
    pub ICC_IAR1_EL1: ICC_IAR1_EL1_Type,
    pub _ICC_IAR1: ICC_IAR1_Type,
    pub ICC_IGRPEN0_EL1: ICC_IGRPEN0_EL1_Type,
    pub _ICC_IGRPEN0: ICC_IGRPEN0_Type,
    pub ICC_IGRPEN1_EL1_NS: ICC_IGRPEN1_EL1_Type,
    pub _ICC_IGRPEN1_NS: ICC_IGRPEN1_Type,
    pub ICC_IGRPEN1_EL1_S: ICC_IGRPEN1_EL1_Type,
    pub _ICC_IGRPEN1_S: ICC_IGRPEN1_Type,
    pub _ICC_PMR: ICC_PMR_Type,
    pub ICC_RPR_EL1: ICC_RPR_EL1_Type,
    pub _ICC_RPR: ICC_RPR_Type,
    pub ICC_SGI0R_EL1: ICC_SGI0R_EL1_Type,
    pub _ICC_SGI0R: ICC_SGI0R_Type,
    pub ICC_SGI1R_EL1: ICC_SGI1R_EL1_Type,
    pub _ICC_SGI1R: ICC_SGI1R_Type,
    pub ICC_SRE_EL1_NS: ICC_SRE_EL1_Type,
    pub _ICC_SRE_NS: ICC_SRE_Type,
    pub ICC_SRE_EL1_S: ICC_SRE_EL1_Type,
    pub _ICC_SRE_S: ICC_SRE_Type,
    pub ICH_EISR_EL2: ICH_EISR_EL2_Type,
    pub _ICH_EISR: ICH_EISR_Type,
    pub ICH_ELRSR_EL2: ICH_ELRSR_EL2_Type,
    pub _ICH_ELRSR: ICH_ELRSR_Type,
    pub ICH_HCR_EL2: ICH_HCR_EL2_Type,
    pub _ICH_HCR: ICH_HCR_Type,
    pub ICH_MISR_EL2: ICH_MISR_EL2_Type,
    pub _ICH_MISR: ICH_MISR_Type,
    pub ICH_VMCR_EL2: ICH_VMCR_EL2_Type,
    pub _ICH_VMCR: ICH_VMCR_Type,
    pub ICH_VTR_EL2: ICH_VTR_EL2_Type,
    pub _ICH_VTR: ICH_VTR_Type,
    pub ICV_BPR0_EL1: ICV_BPR0_EL1_Type,
    pub _ICV_BPR0: ICV_BPR0_Type,
    pub ICV_BPR1_EL1: ICV_BPR1_EL1_Type,
    pub _ICV_BPR1: ICV_BPR1_Type,
    pub ICV_CTLR_EL1: ICV_CTLR_EL1_Type,
    pub _ICV_CTLR: ICV_CTLR_Type,
    pub ICV_DIR_EL1: ICV_DIR_EL1_Type,
    pub _ICV_DIR: ICV_DIR_Type,
    pub ICV_EOIR0_EL1: ICV_EOIR0_EL1_Type,
    pub _ICV_EOIR0: ICV_EOIR0_Type,
    pub ICV_EOIR1_EL1: ICV_EOIR1_EL1_Type,
    pub _ICV_EOIR1: ICV_EOIR1_Type,
    pub ICV_HPPIR0_EL1: ICV_HPPIR0_EL1_Type,
    pub _ICV_HPPIR0: ICV_HPPIR0_Type,
    pub ICV_HPPIR1_EL1: ICV_HPPIR1_EL1_Type,
    pub _ICV_HPPIR1: ICV_HPPIR1_Type,
    pub ICV_IAR0_EL1: ICV_IAR0_EL1_Type,
    pub _ICV_IAR0: ICV_IAR0_Type,
    pub ICV_IAR1_EL1: ICV_IAR1_EL1_Type,
    pub _ICV_IAR1: ICV_IAR1_Type,
    pub ICV_IGRPEN0_EL1: ICV_IGRPEN0_EL1_Type,
    pub _ICV_IGRPEN0: ICV_IGRPEN0_Type,
    pub ICV_IGRPEN1_EL1: ICV_IGRPEN1_EL1_Type,
    pub _ICV_IGRPEN1: ICV_IGRPEN1_Type,
    pub ICV_PMR_EL1: ICV_PMR_EL1_Type,
    pub _ICV_PMR: ICV_PMR_Type,
    pub ICV_RPR_EL1: ICV_RPR_EL1_Type,
    pub _ICV_RPR: ICV_RPR_Type,
    pub ID_AFR0_EL1: ID_AFR0_EL1_Type,
    pub _ID_AFR0: ID_AFR0_Type,
    pub ID_DFR1_EL1: ID_DFR1_EL1_Type,
    pub _ID_DFR1: ID_DFR1_Type,
    pub ID_ISAR0_EL1: ID_ISAR0_EL1_Type,
    pub _ID_ISAR0: ID_ISAR0_Type,
    pub ID_ISAR5_EL1: ID_ISAR5_EL1_Type,
    pub _ID_ISAR5: ID_ISAR5_Type,
    pub ID_MMFR5_EL1: ID_MMFR5_EL1_Type,
    pub _ID_MMFR5: ID_MMFR5_Type,
    pub ID_PFR2_EL1: ID_PFR2_EL1_Type,
    pub _ID_PFR2: ID_PFR2_Type,
    pub ISR_EL1: ISR_EL1_Type,
    pub _ISR: ISR_Type,
    pub MPIDR_EL1: MPIDR_EL1_Type,
    pub _MPIDR: MPIDR_Type,
    pub MVFR2_EL1: MVFR2_EL1_Type,
    pub _MVFR2: MVFR2_Type,
    pub PAR_NS: PAR_Type,
    pub PAR_S: PAR_Type,
    pub _PMCNTEN: PMCNTEN_Type,
    pub PMINTENSET_EL1: PMINTENSET_EL1_Type,
    pub _PMINTEN: PMINTEN_Type,
    pub _PMOVS: PMOVS_Type,
    pub PMSELR_EL0: PMSELR_EL0_Type,
    pub _PMSELR: PMSELR_Type,
    pub PMSWINC_EL0: PMSWINC_EL0_Type,
    pub _PMSWINC: PMSWINC_Type,
    pub PMUSERENR_EL0: PMUSERENR_EL0_Type,
    pub _PMUSERENR: PMUSERENR_Type,
    pub PRRR_S: PRRR_Type,
    pub _PRRR_NS: PRRR_Type,
    pub RMR_EL1: RMR_EL1_Type,
    pub RMR_EL3: RMR_EL3_Type,
    pub _RMR: RMR_Type,
    pub TRFCR_EL1: TRFCR_EL1_Type,
    pub _TRFCR: TRFCR_Type,
    pub TTBCR2_S: TTBCR2_Type,
    pub _TTBCR2_NS: TTBCR2_Type,
    pub VMPIDR_EL2: VMPIDR_EL2_Type,
    pub _VMPIDR: VMPIDR_Type,
    pub VPIDR_EL2: VPIDR_EL2_Type,
    pub _VPIDR: VPIDR_Type,
    pub _VTCR: VTCR_Type,
    pub DBGDEVID: DBGDEVID_Type,
    pub FPSID: FPSID_Type,
    pub ID_DFR0_EL1: ID_DFR0_EL1_Type,
    pub TLBTR: TLBTR_Type,
    pub __exclusive_granule_size: BitVector<4>,
    pub __num_ctx_breakpoints: i128,
    pub _CLIDR: CLIDR_Type,
    pub _ID_DFR0: ID_DFR0_Type,
    pub ID_ISAR1_EL1: ID_ISAR1_EL1_Type,
    pub _ID_ISAR1: ID_ISAR1_Type,
    pub ID_ISAR2_EL1: ID_ISAR2_EL1_Type,
    pub _ID_ISAR2: ID_ISAR2_Type,
    pub ID_ISAR3_EL1: ID_ISAR3_EL1_Type,
    pub _ID_ISAR3: ID_ISAR3_Type,
    pub ID_ISAR4_EL1: ID_ISAR4_EL1_Type,
    pub _ID_ISAR4: ID_ISAR4_Type,
    pub ID_ISAR6_EL1: ID_ISAR6_EL1_Type,
    pub _ID_ISAR6: ID_ISAR6_Type,
    pub ID_MMFR0_EL1: ID_MMFR0_EL1_Type,
    pub _ID_MMFR0: ID_MMFR0_Type,
    pub ID_MMFR1_EL1: ID_MMFR1_EL1_Type,
    pub _ID_MMFR1: ID_MMFR1_Type,
    pub ID_MMFR2_EL1: ID_MMFR2_EL1_Type,
    pub _ID_MMFR2: ID_MMFR2_Type,
    pub ID_MMFR3_EL1: ID_MMFR3_EL1_Type,
    pub _ID_MMFR3: ID_MMFR3_Type,
    pub ID_MMFR4_EL1: ID_MMFR4_EL1_Type,
    pub _ID_MMFR4: ID_MMFR4_Type,
    pub ID_PFR0_EL1: ID_PFR0_EL1_Type,
    pub _ID_PFR0: ID_PFR0_Type,
    pub ID_PFR1_EL1: ID_PFR1_EL1_Type,
    pub _ID_PFR1: ID_PFR1_Type,
    pub MIDR_EL1: MIDR_EL1_Type,
    pub _MIDR: MIDR_Type,
    pub MVFR0_EL1: MVFR0_EL1_Type,
    pub _MVFR0: MVFR0_Type,
    pub MVFR1_EL1: MVFR1_EL1_Type,
    pub _MVFR1: MVFR1_Type,
    pub NMRR_S: NMRR_Type,
    pub _NMRR_NS: NMRR_Type,
    pub PMCEID0_EL0: PMCEID0_EL0_Type,
    pub _PMCEID0: PMCEID0_Type,
    pub PMCEID1_EL0: PMCEID1_EL0_Type,
    pub _PMCEID1: PMCEID1_Type,
    pub _PMCEID2: PMCEID2_Type,
    pub _PMCEID3: PMCEID3_Type,
    pub ACCDATA_EL1: ACCDATA_EL1_Type,
    pub AMCG1IDR_EL0: AMCG1IDR_EL0_Type,
    pub BRBINFINJ_EL1: BRBINFINJ_EL1_Type,
    pub CNTFRQ_EL0: CNTFRQ_EL0_Type,
    pub CNTHPS_TVAL_EL2: CNTHPS_TVAL_EL2_Type,
    pub CNTHP_TVAL_EL2: CNTHP_TVAL_EL2_Type,
    pub CNTHVS_TVAL_EL2: CNTHVS_TVAL_EL2_Type,
    pub CNTHV_TVAL_EL2: CNTHV_TVAL_EL2_Type,
    pub CNTPS_TVAL_EL1: CNTPS_TVAL_EL1_Type,
    pub CNTP_TVAL_EL0: CNTP_TVAL_EL0_Type,
    pub CNTV_TVAL_EL0: CNTV_TVAL_EL0_Type,
    pub DACR32_EL2: DACR32_EL2_Type,
    pub DBGDTRRX_EL0: DBGDTRRX_EL0_Type,
    pub DBGDTRTX_EL0: DBGDTRTX_EL0_Type,
    pub DCZID_EL0: DCZID_EL0_Type,
    pub GMID_EL1: GMID_EL1_Type,
    pub HAFGRTR_EL2: HAFGRTR_EL2_Type,
    pub HDFGRTR2_EL2: HDFGRTR2_EL2_Type,
    pub HDFGRTR_EL2: HDFGRTR_EL2_Type,
    pub HDFGWTR2_EL2: HDFGWTR2_EL2_Type,
    pub HDFGWTR_EL2: HDFGWTR_EL2_Type,
    pub HFGITR2_EL2: HFGITR2_EL2_Type,
    pub HFGRTR2_EL2: HFGRTR2_EL2_Type,
    pub HFGRTR_EL2: HFGRTR_EL2_Type,
    pub HFGWTR2_EL2: HFGWTR2_EL2_Type,
    pub HFGWTR_EL2: HFGWTR_EL2_Type,
    pub ICC_CTLR_EL3: ICC_CTLR_EL3_Type,
    pub ICC_IGRPEN1_EL3: ICC_IGRPEN1_EL3_Type,
    pub ICC_NMIAR1_EL1: ICC_NMIAR1_EL1_Type,
    pub ICC_SRE_EL3: ICC_SRE_EL3_Type,
    pub ICV_NMIAR1_EL1: ICV_NMIAR1_EL1_Type,
    pub ID_AA64AFR0_EL1: ID_AA64AFR0_EL1_Type,
    pub ID_AA64AFR1_EL1: ID_AA64AFR1_EL1_Type,
    pub ID_AA64ISAR0_EL1: ID_AA64ISAR0_EL1_Type,
    pub ID_AA64ISAR2_EL1: ID_AA64ISAR2_EL1_Type,
    pub ID_AA64MMFR0_EL1: ID_AA64MMFR0_EL1_Type,
    pub ID_AA64MMFR2_EL1: ID_AA64MMFR2_EL1_Type,
    pub ID_AA64MMFR3_EL1: ID_AA64MMFR3_EL1_Type,
    pub ID_AA64MMFR4_EL1: ID_AA64MMFR4_EL1_Type,
    pub ID_AA64PFR1_EL1: ID_AA64PFR1_EL1_Type,
    pub ID_AA64PFR2_EL1: ID_AA64PFR2_EL1_Type,
    pub ID_AA64SMFR0_EL1: ID_AA64SMFR0_EL1_Type,
    pub ID_AA64ZFR0_EL1: ID_AA64ZFR0_EL1_Type,
    pub LORC_EL1: LORC_EL1_Type,
    pub LOREA_EL1: LOREA_EL1_Type,
    pub LORID_EL1: LORID_EL1_Type,
    pub LORN_EL1: LORN_EL1_Type,
    pub LORSA_EL1: LORSA_EL1_Type,
    pub MDSELR_EL1: MDSELR_EL1_Type,
    pub MECIDR_EL2: MECIDR_EL2_Type,
    pub OSDTRRX_EL1: OSDTRRX_EL1_Type,
    pub OSDTRTX_EL1: OSDTRTX_EL1_Type,
    pub OSLAR_EL1: OSLAR_EL1_Type,
    pub PMBIDR_EL1: PMBIDR_EL1_Type,
    pub PMECR_EL1: PMECR_EL1_Type,
    pub PMMIR_EL1: PMMIR_EL1_Type,
    pub PMSEVFR_EL1: PMSEVFR_EL1_Type,
    pub PMSFCR_EL1: PMSFCR_EL1_Type,
    pub PMSICR_EL1: PMSICR_EL1_Type,
    pub PMSIRR_EL1: PMSIRR_EL1_Type,
    pub PMSLATFR_EL1: PMSLATFR_EL1_Type,
    pub PMSNEVFR_EL1: PMSNEVFR_EL1_Type,
    pub PMSSCR_EL1: PMSSCR_EL1_Type,
    pub PMUACR_EL1: PMUACR_EL1_Type,
    pub PMXEVCNTR_EL0: PMXEVCNTR_EL0_Type,
    pub PMZR_EL0: PMZR_EL0_Type,
    pub SMIDR_EL1: SMIDR_EL1_Type,
    pub SMPRI_EL1: SMPRI_EL1_Type,
    pub SPMSELR_EL0: SPMSELR_EL0_Type,
    pub _PAR_EL1: PAR_EL1_Type,
    pub AMDEVARCH: AMDEVARCH_Type,
    pub AMEVTYPER0_EL0: [AMEVTYPER0_EL0_Type; (4 as usize)],
    pub AMIIDR: AMIIDR_Type,
    pub CFG_MPIDR: BitVector<32>,
    pub CNTFID0: CNTFID0_Type,
    pub CTIDEVARCH: CTIDEVARCH_Type,
    pub EDDEVARCH: EDDEVARCH_Type,
    pub GICD_TYPER: GICD_TYPER_Type,
    pub ID_AA64DFR0_EL1: ID_AA64DFR0_EL1_Type,
    pub ID_AA64DFR1_EL1: ID_AA64DFR1_EL1_Type,
    pub ID_AA64ISAR1_EL1: ID_AA64ISAR1_EL1_Type,
    pub ID_AA64MMFR1_EL1: ID_AA64MMFR1_EL1_Type,
    pub ID_AA64PFR0_EL1: ID_AA64PFR0_EL1_Type,
    pub __CNTbase_frequency: BitVector<32>,
    pub __dczid_log2_block_size: i128,
    pub __gmid_log2_block_size: i128,
    pub __mecid_width: BitVector<4>,
    pub __mpam_has_altsp: bool,
    pub __rme_l0gptsz: BitVector<4>,
    pub __supported_va_size: i128,
    pub __g1_activity_monitor_implemented: BitVector<16>,
    pub __g1_activity_monitor_offset_implemented: BitVector<16>,
    pub PMEVCNTR_EL0: [PMEVCNTR_EL0_Type; (32 as usize)],
    pub PMCCNTR_EL0: PMCCNTR_EL0_Type,
    pub PMBPTR_EL1: PMBPTR_EL1_Type,
    pub PMSDSFR_EL1: PMSDSFR_EL1_Type,
    pub SPERecordData: [BitVector<8>; (64 as usize)],
    pub SPESampleCounter: [i128; (32 as usize)],
    pub BRBSRCINJ_EL1: BRBSRCINJ_EL1_Type,
    pub BRBTGTINJ_EL1: BRBTGTINJ_EL1_Type,
    pub _MAIR0_NS: MAIR0_Type,
    pub _MAIR0_S: MAIR0_Type,
    pub _MAIR1_NS: MAIR1_Type,
    pub _MAIR1_S: MAIR1_Type,
    pub _HMAIR0: HMAIR0_Type,
    pub _HMAIR1: HMAIR1_Type,
    pub _DACR_NS: DACR_Type,
    pub DACR_S: DACR_Type,
    pub _DBGDTR_EL0: DBGDTR_EL0_Type,
    pub __CTIBase: BitVector<56>,
    pub __CNTControlBase: BitVector<56>,
    pub __ExtDebugBase: BitVector<56>,
    pub __GICCPUInterfaceBase: BitVector<56>,
    pub __GICDistBase: BitVector<56>,
    pub __GICITSControlBase: BitVector<56>,
    pub __PMUBase: BitVector<56>,
    pub __syncAbortOnReadNormCache: bool,
    pub __syncAbortOnReadNormNonCache: bool,
    pub __syncAbortOnDeviceRead: bool,
    pub __syncAbortOnSoRead: bool,
    pub __syncAbortOnSoWrite: bool,
    pub __syncAbortOnPrefetch: bool,
    pub __syncAbortOnTTWCache: bool,
    pub __syncAbortOnTTWNonCache: bool,
    pub __syncAbortOnWriteNormCache: bool,
    pub __syncAbortOnWriteNormNonCache: bool,
    pub __syncAbortOnDeviceWrite: bool,
    pub __unpred_tsize_aborts: bool,
    pub __ignore_rvbar_in_aarch32: bool,
    pub __trickbox_enabled: bool,
    pub __mops_forward_copy: bool,
    pub __DBG_ROM_ADDR: BitVector<56>,
    pub CFG_RMR_AA64: BitVector<1>,
    pub ZCR_EL3_LEN_VALUE: i128,
    pub CPTR_EL3_EZ_VALUE: i128,
    pub CPTR_EL3_ESM_VALUE: i128,
    pub SMCR_EL3_LEN_VALUE: i128,
    pub __has_spe_pseudo_cycles: bool,
    pub HEAP_BASE: BitVector<64>,
    pub HEAP_LIMIT: BitVector<64>,
    pub STACK_BASE: BitVector<64>,
    pub STACK_LIMIT: BitVector<64>,
    pub _DBGBCR: [DBGBCR_Type; (16 as usize)],
    pub _DBGBVR: [DBGBVR_Type; (16 as usize)],
    pub _DBGBXVR: [DBGBXVR_Type; (16 as usize)],
    pub _DBGWCR: [DBGWCR_Type; (16 as usize)],
    pub _DBGWVR: [DBGWVR_Type; (16 as usize)],
    pub _PMEVCNTR: [PMEVCNTR_Type; (31 as usize)],
    pub _PMOVSSET: PMOVSSET_Type,
    pub _PMCCNTR: PMCCNTR_Type,
    pub ICH_LR_EL2: [ICH_LR_EL2_Type; (16 as usize)],
    pub ICC_AP0R_EL1: [ICC_AP0R_EL1_Type; (4 as usize)],
    pub AIDR_EL1: AIDR_EL1_Type,
    pub REVIDR_EL1: REVIDR_EL1_Type,
    pub TPIDR_EL2: TPIDR_EL2_Type,
    pub ACTLR_EL2: ACTLR_EL2_Type,
    pub SPMACCESSR_EL1: SPMACCESSR_EL1_Type,
    pub ICH_AP1R_EL2: [ICH_AP1R_EL2_Type; (4 as usize)],
    pub PMEVCNTSVR_EL1: [PMEVCNTSVR_EL1_Type; (31 as usize)],
    pub ICH_AP0R_EL2: [ICH_AP0R_EL2_Type; (4 as usize)],
    pub TPIDR_EL1: TPIDR_EL1_Type,
    pub ACTLR_EL1: ACTLR_EL1_Type,
    pub ICC_AP1R_EL1_S: [ICC_AP1R_EL1_Type; (4 as usize)],
    pub ICC_AP1R_EL1_NS: [ICC_AP1R_EL1_Type; (4 as usize)],
    pub AFSR1_EL1: AFSR1_EL1_Type,
    pub PMCCNTSVR_EL1: PMCCNTSVR_EL1_Type,
    pub AMAIR2_EL2: AMAIR2_EL2_Type,
    pub ICV_AP0R_EL1: [ICV_AP0R_EL1_Type; (4 as usize)],
    pub AFSR0_EL3: AFSR0_EL3_Type,
    pub AFSR1_EL2: AFSR1_EL2_Type,
    pub AFSR0_EL1: AFSR0_EL1_Type,
    pub SPMACCESSR_EL2: SPMACCESSR_EL2_Type,
    pub PMICNTSVR_EL1: PMICNTSVR_EL1_Type,
    pub SCXTNUM_EL2: SCXTNUM_EL2_Type,
    pub PMIAR_EL1: PMIAR_EL1_Type,
    pub TPIDR_EL3: TPIDR_EL3_Type,
    pub ACTLR_EL3: ACTLR_EL3_Type,
    pub AMAIR_EL2: AMAIR_EL2_Type,
    pub AMAIR_EL3: AMAIR_EL3_Type,
    pub SCXTNUM_EL1: SCXTNUM_EL1_Type,
    pub TPIDRRO_EL0: TPIDRRO_EL0_Type,
    pub AMAIR_EL1: AMAIR_EL1_Type,
    pub SCXTNUM_EL0: SCXTNUM_EL0_Type,
    pub TPIDR2_EL0: TPIDR2_EL0_Type,
    pub SCXTNUM_EL3: SCXTNUM_EL3_Type,
    pub TPIDR_EL0: TPIDR_EL0_Type,
    pub RNDRRS: RNDRRS_Type,
    pub SMPRIMAP_EL2: SMPRIMAP_EL2_Type,
    pub RNDR: RNDR_Type,
    pub HACR_EL2: HACR_EL2_Type,
    pub AMAIR2_EL3: AMAIR2_EL3_Type,
    pub AFSR1_EL3: AFSR1_EL3_Type,
    pub ICV_AP1R_EL1: [ICV_AP1R_EL1_Type; (4 as usize)],
    pub AFSR0_EL2: AFSR0_EL2_Type,
    pub AMAIR2_EL1: AMAIR2_EL1_Type,
    pub SPMACCESSR_EL3: SPMACCESSR_EL3_Type,
    pub _CNTVOFF: CNTVOFF_Type,
    pub _CNTV_CVAL: CNTV_CVAL_Type,
    pub AMEVCNTR1_EL0: [AMEVCNTR1_EL0_Type; (16 as usize)],
    pub AMEVCNTVOFF0_EL2: [AMEVCNTVOFF0_EL2_Type; (16 as usize)],
    pub AMEVCNTR0: [AMEVCNTR0_Type; (4 as usize)],
    pub _AMEVCNTR1: [AMEVCNTR1_Type; (16 as usize)],
    pub _AMEVCNTR0_EL0: [AMEVCNTR0_EL0_Type; (4 as usize)],
    pub AMEVCNTVOFF1_EL2: [AMEVCNTVOFF1_EL2_Type; (16 as usize)],
    pub AMEVTYPER1_EL0: [AMEVTYPER1_EL0_Type; (16 as usize)],
    pub ERXADDR_EL1: ERXADDR_EL1_Type,
    pub ERXMISC0_EL1: ERXMISC0_EL1_Type,
    pub ERXPFGCDN_EL1: ERXPFGCDN_EL1_Type,
    pub ERXPFGF_EL1: ERXPFGF_EL1_Type,
    pub ERXMISC1_EL1: ERXMISC1_EL1_Type,
    pub ERXCTLR_EL1: ERXCTLR_EL1_Type,
    pub ERXSTATUS_EL1: ERXSTATUS_EL1_Type,
    pub ERXFR_EL1: ERXFR_EL1_Type,
    pub ERXGSR_EL1: ERXGSR_EL1_Type,
    pub ERXMISC3_EL1: ERXMISC3_EL1_Type,
    pub ERXMISC2_EL1: ERXMISC2_EL1_Type,
    pub ERXPFGCTL_EL1: ERXPFGCTL_EL1_Type,
    pub BRBINF_EL1: [BRBINF_EL1_Type; (32 as usize)],
    pub BRBTGT_EL1: [BRBTGT_EL1_Type; (32 as usize)],
    pub BRBSRC_EL1: [BRBSRC_EL1_Type; (32 as usize)],
    pub AIFSR_S: AIFSR_Type,
    pub _AIFSR_NS: AIFSR_Type,
    pub _HACR: HACR_Type,
    pub _HACTLR: HACTLR_Type,
    pub TCMTR: TCMTR_Type,
    pub _ICV_AP0R: [ICV_AP0R_Type; (4 as usize)],
    pub TPIDRURW_S: TPIDRURW_Type,
    pub _TPIDRURW_NS: TPIDRURW_Type,
    pub _ICV_AP1R: [ICV_AP1R_Type; (4 as usize)],
    pub ADFSR_S: ADFSR_Type,
    pub _ADFSR_NS: ADFSR_Type,
    pub _REVIDR: REVIDR_Type,
    pub TPIDRURO_S: TPIDRURO_Type,
    pub _TPIDRURO_NS: TPIDRURO_Type,
    pub _DBGDTRTXext: DBGDTRTXext_Type,
    pub ACTLR_S: ACTLR_Type,
    pub _ACTLR_NS: ACTLR_Type,
    pub DBGOSLAR: DBGOSLAR_Type,
    pub _DBGDTRRXext: DBGDTRRXext_Type,
    pub _ICC_AP1R_S: [ICC_AP1R_Type; (4 as usize)],
    pub _ICC_AP1R_NS: [ICC_AP1R_Type; (4 as usize)],
    pub _ICH_AP1R: [ICH_AP1R_Type; (4 as usize)],
    pub AMAIR0_S: AMAIR0_Type,
    pub _AMAIR0_NS: AMAIR0_Type,
    pub _HAMAIR0: HAMAIR0_Type,
    pub _ICH_AP0R: [ICH_AP0R_Type; (4 as usize)],
    pub _AIDR: AIDR_Type,
    pub _ICH_LR: [ICH_LR_Type; (16 as usize)],
    pub _HADFSR: HADFSR_Type,
    pub _CNTFRQ: CNTFRQ_Type,
    pub _PMINTENSET: PMINTENSET_Type,
    pub _DBGOSECCR: DBGOSECCR_Type,
    pub ACTLR2_S: ACTLR2_Type,
    pub _ACTLR2_NS: ACTLR2_Type,
    pub _HAMAIR1: HAMAIR1_Type,
    pub _HAIFSR: HAIFSR_Type,
    pub _ICC_AP0R: [ICC_AP0R_Type; (4 as usize)],
    pub TPIDRPRW_S: TPIDRPRW_Type,
    pub _TPIDRPRW_NS: TPIDRPRW_Type,
    pub _DBGDTRRXint: DBGDTRRXint_Type,
    pub _HTPIDR: HTPIDR_Type,
    pub _DBGDTRTXint: DBGDTRTXint_Type,
    pub AMAIR1_S: AMAIR1_Type,
    pub _AMAIR1_NS: AMAIR1_Type,
    pub _ICH_LRC: [ICH_LRC_Type; (16 as usize)],
    pub _AMEVTYPER0: [AMEVTYPER0_Type; (4 as usize)],
    pub _AMEVTYPER1: [AMEVTYPER1_Type; (16 as usize)],
    pub _ERXFR2: ERXFR2_Type,
    pub _ERXMISC2: ERXMISC2_Type,
    pub _ERXFR: ERXFR_Type,
    pub _ERXADDR: ERXADDR_Type,
    pub _ERXMISC0: ERXMISC0_Type,
    pub _ERXMISC5: ERXMISC5_Type,
    pub _ERXCTLR2: ERXCTLR2_Type,
    pub _ERXMISC1: ERXMISC1_Type,
    pub _ERXCTLR: ERXCTLR_Type,
    pub _ERXMISC6: ERXMISC6_Type,
    pub _ERXMISC4: ERXMISC4_Type,
    pub _ERXADDR2: ERXADDR2_Type,
    pub _ERXMISC7: ERXMISC7_Type,
    pub _ERXMISC3: ERXMISC3_Type,
    pub _ERXSTATUS: ERXSTATUS_Type,
    pub _HACTLR2: HACTLR2_Type,
    pub __emulator_termination_opcode: Option<BitVector<32>>,
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {}

/// Initialize all registers.
///
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers(core_ctx: &mut Core) {
    core_ctx.FEAT_AA32EL0_IMPLEMENTED = true;
    core_ctx.FEAT_AA32EL1_IMPLEMENTED = false;
    core_ctx.FEAT_AA32EL2_IMPLEMENTED = false;
    core_ctx.FEAT_AA32EL3_IMPLEMENTED = false;
    core_ctx.FEAT_AA64EL0_IMPLEMENTED = true;
    core_ctx.FEAT_AA64EL1_IMPLEMENTED = true;
    core_ctx.FEAT_AA64EL2_IMPLEMENTED = true;
    core_ctx.FEAT_AA64EL3_IMPLEMENTED = true;
    core_ctx.FEAT_EL0_IMPLEMENTED = true;
    core_ctx.FEAT_EL1_IMPLEMENTED = true;
    core_ctx.FEAT_EL2_IMPLEMENTED = true;
    core_ctx.FEAT_EL3_IMPLEMENTED = true;
    core_ctx.FEAT_AES_IMPLEMENTED = true;
    core_ctx.FEAT_AdvSIMD_IMPLEMENTED = true;
    core_ctx.FEAT_CSV2_1p1_IMPLEMENTED = true;
    core_ctx.FEAT_CSV2_1p2_IMPLEMENTED = true;
    core_ctx.FEAT_CSV2_2_IMPLEMENTED = true;
    core_ctx.FEAT_CSV2_3_IMPLEMENTED = true;
    core_ctx.FEAT_DoubleLock_IMPLEMENTED = true;
    core_ctx.FEAT_ETMv4_IMPLEMENTED = false;
    core_ctx.FEAT_ETMv4p1_IMPLEMENTED = true;
    core_ctx.FEAT_ETMv4p2_IMPLEMENTED = true;
    core_ctx.FEAT_ETMv4p3_IMPLEMENTED = true;
    core_ctx.FEAT_ETMv4p4_IMPLEMENTED = true;
    core_ctx.FEAT_ETMv4p5_IMPLEMENTED = true;
    core_ctx.FEAT_ETMv4p6_IMPLEMENTED = true;
    core_ctx.FEAT_ETS2_IMPLEMENTED = true;
    core_ctx.FEAT_FP_IMPLEMENTED = true;
    core_ctx.FEAT_GICv3_IMPLEMENTED = true;
    core_ctx.FEAT_GICv3_LEGACY_IMPLEMENTED = true;
    core_ctx.FEAT_GICv3_TDIR_IMPLEMENTED = true;
    core_ctx.FEAT_GICv3p1_IMPLEMENTED = true;
    core_ctx.FEAT_GICv4_IMPLEMENTED = true;
    core_ctx.FEAT_GICv4p1_IMPLEMENTED = true;
    core_ctx.FEAT_IVIPT_IMPLEMENTED = true;
    core_ctx.FEAT_PCSRv8_IMPLEMENTED = true;
    core_ctx.FEAT_PMULL_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_EXT_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_EXT32_IMPLEMENTED = true;
    core_ctx.FEAT_SHA1_IMPLEMENTED = true;
    core_ctx.FEAT_SHA256_IMPLEMENTED = true;
    core_ctx.FEAT_TRC_EXT_IMPLEMENTED = true;
    core_ctx.FEAT_TRC_SR_IMPLEMENTED = true;
    core_ctx.FEAT_nTLBPA_IMPLEMENTED = true;
    core_ctx.FEAT_CRC32_IMPLEMENTED = true;
    core_ctx.FEAT_Debugv8p1_IMPLEMENTED = true;
    core_ctx.FEAT_HAFDBS_IMPLEMENTED = true;
    core_ctx.FEAT_HPDS_IMPLEMENTED = true;
    core_ctx.FEAT_LOR_IMPLEMENTED = true;
    core_ctx.FEAT_LSE_IMPLEMENTED = true;
    core_ctx.FEAT_PAN_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3p1_IMPLEMENTED = true;
    core_ctx.FEAT_RDM_IMPLEMENTED = true;
    core_ctx.FEAT_VHE_IMPLEMENTED = true;
    core_ctx.FEAT_VMID16_IMPLEMENTED = true;
    core_ctx.FEAT_AA32BF16_IMPLEMENTED = true;
    core_ctx.FEAT_AA32HPD_IMPLEMENTED = true;
    core_ctx.FEAT_AA32I8MM_IMPLEMENTED = true;
    core_ctx.FEAT_ASMv8p2_IMPLEMENTED = true;
    core_ctx.FEAT_DPB_IMPLEMENTED = true;
    core_ctx.FEAT_Debugv8p2_IMPLEMENTED = true;
    core_ctx.FEAT_EDHSR_IMPLEMENTED = true;
    core_ctx.FEAT_F32MM_IMPLEMENTED = true;
    core_ctx.FEAT_F64MM_IMPLEMENTED = true;
    core_ctx.FEAT_FP16_IMPLEMENTED = true;
    core_ctx.FEAT_HPDS2_IMPLEMENTED = true;
    core_ctx.FEAT_I8MM_IMPLEMENTED = true;
    core_ctx.FEAT_IESB_IMPLEMENTED = true;
    core_ctx.FEAT_LPA_IMPLEMENTED = true;
    core_ctx.FEAT_LSMAOC_IMPLEMENTED = true;
    core_ctx.FEAT_LVA_IMPLEMENTED = true;
    core_ctx.FEAT_MPAM_IMPLEMENTED = true;
    core_ctx.FEAT_PAN2_IMPLEMENTED = true;
    core_ctx.FEAT_PCSRv8p2_IMPLEMENTED = true;
    core_ctx.FEAT_RAS_IMPLEMENTED = true;
    core_ctx.FEAT_SHA3_IMPLEMENTED = true;
    core_ctx.FEAT_SHA512_IMPLEMENTED = true;
    core_ctx.FEAT_SM3_IMPLEMENTED = true;
    core_ctx.FEAT_SM4_IMPLEMENTED = true;
    core_ctx.FEAT_SPE_IMPLEMENTED = true;
    core_ctx.FEAT_SVE_IMPLEMENTED = true;
    core_ctx.FEAT_TTCNP_IMPLEMENTED = true;
    core_ctx.FEAT_UAO_IMPLEMENTED = true;
    core_ctx.FEAT_VPIPT_IMPLEMENTED = true;
    core_ctx.FEAT_XNX_IMPLEMENTED = true;
    core_ctx.FEAT_CCIDX_IMPLEMENTED = true;
    core_ctx.FEAT_CONSTPACFIELD_IMPLEMENTED = false;
    core_ctx.FEAT_EPAC_IMPLEMENTED = false;
    core_ctx.FEAT_FCMA_IMPLEMENTED = true;
    core_ctx.FEAT_FPAC_IMPLEMENTED = true;
    core_ctx.FEAT_FPACCOMBINE_IMPLEMENTED = true;
    core_ctx.FEAT_JSCVT_IMPLEMENTED = true;
    core_ctx.FEAT_LRCPC_IMPLEMENTED = true;
    core_ctx.FEAT_NV_IMPLEMENTED = true;
    core_ctx.FEAT_PACIMP_IMPLEMENTED = false;
    core_ctx.FEAT_PACQARMA3_IMPLEMENTED = false;
    core_ctx.FEAT_PACQARMA5_IMPLEMENTED = true;
    core_ctx.FEAT_PAuth_IMPLEMENTED = true;
    core_ctx.FEAT_SPEv1p1_IMPLEMENTED = true;
    core_ctx.FEAT_AMUv1_IMPLEMENTED = true;
    core_ctx.FEAT_BBM_IMPLEMENTED = true;
    core_ctx.FEAT_CNTSC_IMPLEMENTED = true;
    core_ctx.FEAT_DIT_IMPLEMENTED = true;
    core_ctx.FEAT_Debugv8p4_IMPLEMENTED = true;
    core_ctx.FEAT_DotProd_IMPLEMENTED = true;
    core_ctx.FEAT_DoubleFault_IMPLEMENTED = true;
    core_ctx.FEAT_FHM_IMPLEMENTED = true;
    core_ctx.FEAT_FlagM_IMPLEMENTED = true;
    core_ctx.FEAT_IDST_IMPLEMENTED = true;
    core_ctx.FEAT_LRCPC2_IMPLEMENTED = true;
    core_ctx.FEAT_LSE2_IMPLEMENTED = true;
    core_ctx.FEAT_NV2_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3p4_IMPLEMENTED = true;
    core_ctx.FEAT_RASSAv1p1_IMPLEMENTED = true;
    core_ctx.FEAT_RASv1p1_IMPLEMENTED = true;
    core_ctx.FEAT_S2FWB_IMPLEMENTED = true;
    core_ctx.FEAT_SEL2_IMPLEMENTED = true;
    core_ctx.FEAT_TLBIOS_IMPLEMENTED = true;
    core_ctx.FEAT_TLBIRANGE_IMPLEMENTED = true;
    core_ctx.FEAT_TRF_IMPLEMENTED = true;
    core_ctx.FEAT_TTL_IMPLEMENTED = true;
    core_ctx.FEAT_TTST_IMPLEMENTED = true;
    core_ctx.FEAT_BTI_IMPLEMENTED = true;
    core_ctx.FEAT_CSV2_IMPLEMENTED = true;
    core_ctx.FEAT_CSV3_IMPLEMENTED = true;
    core_ctx.FEAT_DPB2_IMPLEMENTED = true;
    core_ctx.FEAT_E0PD_IMPLEMENTED = true;
    core_ctx.FEAT_EVT_IMPLEMENTED = true;
    core_ctx.FEAT_ExS_IMPLEMENTED = true;
    core_ctx.FEAT_FRINTTS_IMPLEMENTED = true;
    core_ctx.FEAT_FlagM2_IMPLEMENTED = true;
    core_ctx.FEAT_GTG_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_IMPLEMENTED = true;
    core_ctx.FEAT_MTE2_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3p5_IMPLEMENTED = true;
    core_ctx.FEAT_RNG_IMPLEMENTED = true;
    core_ctx.FEAT_RNG_TRAP_IMPLEMENTED = true;
    core_ctx.FEAT_SB_IMPLEMENTED = true;
    core_ctx.FEAT_SPECRES_IMPLEMENTED = true;
    core_ctx.FEAT_SSBS_IMPLEMENTED = true;
    core_ctx.FEAT_SSBS2_IMPLEMENTED = true;
    core_ctx.FEAT_AMUv1p1_IMPLEMENTED = true;
    core_ctx.FEAT_BF16_IMPLEMENTED = true;
    core_ctx.FEAT_DGH_IMPLEMENTED = true;
    core_ctx.FEAT_ECV_IMPLEMENTED = true;
    core_ctx.FEAT_FGT_IMPLEMENTED = true;
    core_ctx.FEAT_HPMN0_IMPLEMENTED = true;
    core_ctx.FEAT_MPAMv0p1_IMPLEMENTED = true;
    core_ctx.FEAT_MPAMv1p1_IMPLEMENTED = true;
    core_ctx.FEAT_MTPMU_IMPLEMENTED = true;
    core_ctx.FEAT_PAuth2_IMPLEMENTED = true;
    core_ctx.FEAT_TWED_IMPLEMENTED = true;
    core_ctx.FEAT_AFP_IMPLEMENTED = true;
    core_ctx.FEAT_EBF16_IMPLEMENTED = true;
    core_ctx.FEAT_HCX_IMPLEMENTED = true;
    core_ctx.FEAT_LPA2_IMPLEMENTED = true;
    core_ctx.FEAT_LS64_IMPLEMENTED = true;
    core_ctx.FEAT_LS64_ACCDATA_IMPLEMENTED = true;
    core_ctx.FEAT_LS64_V_IMPLEMENTED = true;
    core_ctx.FEAT_MTE3_IMPLEMENTED = true;
    core_ctx.FEAT_PAN3_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3p7_IMPLEMENTED = true;
    core_ctx.FEAT_RPRES_IMPLEMENTED = true;
    core_ctx.FEAT_SPEv1p2_IMPLEMENTED = true;
    core_ctx.FEAT_WFxT_IMPLEMENTED = true;
    core_ctx.FEAT_XS_IMPLEMENTED = true;
    core_ctx.FEAT_CMOW_IMPLEMENTED = true;
    core_ctx.FEAT_Debugv8p8_IMPLEMENTED = true;
    core_ctx.FEAT_GICv3_NMI_IMPLEMENTED = true;
    core_ctx.FEAT_HBC_IMPLEMENTED = true;
    core_ctx.FEAT_MOPS_IMPLEMENTED = true;
    core_ctx.FEAT_NMI_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_EXT64_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_TH_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3p8_IMPLEMENTED = true;
    core_ctx.FEAT_SCTLR2_IMPLEMENTED = true;
    core_ctx.FEAT_SPEv1p3_IMPLEMENTED = true;
    core_ctx.FEAT_TCR2_IMPLEMENTED = true;
    core_ctx.FEAT_TIDCP1_IMPLEMENTED = true;
    core_ctx.FEAT_ADERR_IMPLEMENTED = true;
    core_ctx.FEAT_AIE_IMPLEMENTED = true;
    core_ctx.FEAT_ANERR_IMPLEMENTED = true;
    core_ctx.FEAT_CLRBHB_IMPLEMENTED = true;
    core_ctx.FEAT_CSSC_IMPLEMENTED = true;
    core_ctx.FEAT_Debugv8p9_IMPLEMENTED = true;
    core_ctx.FEAT_DoubleFault2_IMPLEMENTED = true;
    core_ctx.FEAT_ECBHB_IMPLEMENTED = true;
    core_ctx.FEAT_FGT2_IMPLEMENTED = true;
    core_ctx.FEAT_HAFT_IMPLEMENTED = true;
    core_ctx.FEAT_LRCPC3_IMPLEMENTED = true;
    core_ctx.FEAT_MTE4_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_ASYM_FAULT_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_ASYNC_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_PERM_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_STORE_ONLY_IMPLEMENTED = true;
    core_ctx.FEAT_MTE_TAGGED_FAR_IMPLEMENTED = true;
    core_ctx.FEAT_PCSRv8p9_IMPLEMENTED = true;
    core_ctx.FEAT_PFAR_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_EDGE_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_ICNTR_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3_SS_IMPLEMENTED = true;
    core_ctx.FEAT_PMUv3p9_IMPLEMENTED = true;
    core_ctx.FEAT_PRFMSLC_IMPLEMENTED = true;
    core_ctx.FEAT_RASSAv2_IMPLEMENTED = true;
    core_ctx.FEAT_RASv2_IMPLEMENTED = true;
    core_ctx.FEAT_RPRFM_IMPLEMENTED = true;
    core_ctx.FEAT_S1PIE_IMPLEMENTED = true;
    core_ctx.FEAT_S1POE_IMPLEMENTED = true;
    core_ctx.FEAT_S2PIE_IMPLEMENTED = true;
    core_ctx.FEAT_S2POE_IMPLEMENTED = true;
    core_ctx.FEAT_SPECRES2_IMPLEMENTED = true;
    core_ctx.FEAT_SPE_CRR_IMPLEMENTED = true;
    core_ctx.FEAT_SPE_FDS_IMPLEMENTED = true;
    core_ctx.FEAT_SPEv1p4_IMPLEMENTED = true;
    core_ctx.FEAT_SPMU_IMPLEMENTED = true;
    core_ctx.FEAT_THE_IMPLEMENTED = true;
    core_ctx.FEAT_DoPD_IMPLEMENTED = true;
    core_ctx.FEAT_ETE_IMPLEMENTED = true;
    core_ctx.FEAT_SVE2_IMPLEMENTED = true;
    core_ctx.FEAT_SVE_AES_IMPLEMENTED = true;
    core_ctx.FEAT_SVE_BitPerm_IMPLEMENTED = true;
    core_ctx.FEAT_SVE_PMULL128_IMPLEMENTED = true;
    core_ctx.FEAT_SVE_SHA3_IMPLEMENTED = true;
    core_ctx.FEAT_SVE_SM4_IMPLEMENTED = true;
    core_ctx.FEAT_TME_IMPLEMENTED = true;
    core_ctx.FEAT_TRBE_IMPLEMENTED = true;
    core_ctx.FEAT_ETEv1p1_IMPLEMENTED = true;
    core_ctx.FEAT_BRBE_IMPLEMENTED = true;
    core_ctx.FEAT_ETEv1p2_IMPLEMENTED = true;
    core_ctx.FEAT_RME_IMPLEMENTED = true;
    core_ctx.FEAT_SME_IMPLEMENTED = true;
    core_ctx.FEAT_SME_F64F64_IMPLEMENTED = true;
    core_ctx.FEAT_SME_FA64_IMPLEMENTED = true;
    core_ctx.FEAT_SME_I16I64_IMPLEMENTED = true;
    core_ctx.FEAT_BRBEv1p1_IMPLEMENTED = true;
    core_ctx.FEAT_MEC_IMPLEMENTED = true;
    core_ctx.FEAT_SME2_IMPLEMENTED = true;
    core_ctx.FEAT_ABLE_IMPLEMENTED = true;
    core_ctx.FEAT_CHK_IMPLEMENTED = true;
    core_ctx.FEAT_D128_IMPLEMENTED = true;
    core_ctx.FEAT_EBEP_IMPLEMENTED = true;
    core_ctx.FEAT_ETEv1p3_IMPLEMENTED = true;
    core_ctx.FEAT_GCS_IMPLEMENTED = true;
    core_ctx.FEAT_ITE_IMPLEMENTED = true;
    core_ctx.FEAT_LSE128_IMPLEMENTED = true;
    core_ctx.FEAT_LVA3_IMPLEMENTED = true;
    core_ctx.FEAT_SEBEP_IMPLEMENTED = true;
    core_ctx.FEAT_SME2p1_IMPLEMENTED = true;
    core_ctx.FEAT_SME_F16F16_IMPLEMENTED = true;
    core_ctx.FEAT_SVE2p1_IMPLEMENTED = true;
    core_ctx.FEAT_SVE_B16B16_IMPLEMENTED = true;
    core_ctx.FEAT_SYSINSTR128_IMPLEMENTED = true;
    core_ctx.FEAT_SYSREG128_IMPLEMENTED = true;
    core_ctx.FEAT_TRBE_EXT_IMPLEMENTED = true;
    core_ctx.FEAT_TRBE_MPAM_IMPLEMENTED = true;
    core_ctx.v8Ap0_IMPLEMENTED = true;
    core_ctx.v8Ap1_IMPLEMENTED = true;
    core_ctx.v8Ap2_IMPLEMENTED = true;
    core_ctx.v8Ap3_IMPLEMENTED = true;
    core_ctx.v8Ap4_IMPLEMENTED = true;
    core_ctx.v8Ap5_IMPLEMENTED = true;
    core_ctx.v8Ap6_IMPLEMENTED = true;
    core_ctx.v8Ap7_IMPLEMENTED = true;
    core_ctx.v8Ap8_IMPLEMENTED = true;
    core_ctx.v8Ap9_IMPLEMENTED = true;
    core_ctx.v9Ap0_IMPLEMENTED = true;
    core_ctx.v9Ap1_IMPLEMENTED = true;
    core_ctx.v9Ap2_IMPLEMENTED = true;
    core_ctx.v9Ap3_IMPLEMENTED = true;
    core_ctx.v9Ap4_IMPLEMENTED = true;
    core_ctx.NUM_AMU_COUNTER_GROUPS = 2;
    core_ctx.NUM_AMU_CG0_MONITORS = 4;
    core_ctx.NUM_AMU_CG1_MONITORS = 16;
    core_ctx.NUM_PMU_COUNTERS = 31;
    core_ctx.NUM_BRBE_RECORDS = 64;
    core_ctx.NUM_GIC_PREEMPTION_BITS = 5;
    core_ctx.NUM_GIC_PRIORITY_BITS = 5;
    core_ctx.NUM_GIC_LIST_REGS = 16;
    core_ctx.NUM_BREAKPOINTS = _reset_NUM_BREAKPOINTS();
    core_ctx.NUM_WATCHPOINTS = _reset_NUM_WATCHPOINTS();
    core_ctx.__apply_effective_shareability = true;
    core_ctx.__cpy_mops_option_a_supported = true;
    core_ctx.__cpyf_mops_option_a_supported = true;
    core_ctx.__empam_force_ns_RAO = false;
    core_ctx.__empam_force_ns_implemented = false;
    core_ctx.__empam_sdeflt_implemented = false;
    core_ctx.__empam_tidr_implemented = false;
    core_ctx.__feat_rpres = true;
    core_ctx.__has_sme_priority_control = true;
    core_ctx.__isb_is_branch = true;
    core_ctx.__mpam_frac = _reset___mpam_frac();
    core_ctx.__mpam_major = _reset___mpam_major();
    core_ctx.__mte_implemented = _reset___mte_implemented();
    core_ctx.__set_mops_option_a_supported = true;
    core_ctx.__setg_mops_option_a_supported = true;
    core_ctx.__sme_only = false;
    core_ctx.__block_bbm_implemented = _reset___block_bbm_implemented();
    core_ctx.__has_sve_extended_bf16 = 2;
    core_ctx.__max_implemented_smeveclen = 512;
    core_ctx.__max_implemented_sveveclen = 2048;
    core_ctx.__supported_pa_size = 56;
    core_ctx.CFG_RVBAR = _reset_CFG_RVBAR();
    core_ctx.__impdef_TG0 = _reset___impdef_TG0();
    core_ctx.__impdef_TG1 = _reset___impdef_TG1();
    core_ctx.__mpam_has_hcr = true;
    core_ctx.__mpam_partid_max = _reset___mpam_partid_max();
    core_ctx.__mpam_pmg_max = _reset___mpam_pmg_max();
    core_ctx.__mpam_vpmr_max = _reset___mpam_vpmr_max();
    core_ctx.__GIC_Active = _reset___GIC_Active();
    core_ctx.__GIC_Pending = _reset___GIC_Pending();
    core_ctx.__tlb_enabled = false;
    core_ctx.__exclusive_granule_size = _reset___exclusive_granule_size();
    core_ctx.__num_ctx_breakpoints = _reset___num_ctx_breakpoints(core_ctx);
    core_ctx.CFG_MPIDR = _reset_CFG_MPIDR();
    core_ctx.__CNTbase_frequency = _reset___CNTbase_frequency();
    core_ctx.__dczid_log2_block_size = _reset___dczid_log2_block_size();
    core_ctx.__gmid_log2_block_size = _reset___gmid_log2_block_size();
    core_ctx.__mecid_width = _reset___mecid_width();
    core_ctx.__mpam_has_altsp = true;
    core_ctx.__rme_l0gptsz = _reset___rme_l0gptsz();
    core_ctx.__supported_va_size = 56;
    core_ctx.__g1_activity_monitor_implemented = _reset___g1_activity_monitor_implemented();
    core_ctx.__g1_activity_monitor_offset_implemented =
        _reset___g1_activity_monitor_offset_implemented();
    core_ctx.__CTIBase = _reset___CTIBase();
    core_ctx.__CNTControlBase = _reset___CNTControlBase();
    core_ctx.__ExtDebugBase = _reset___ExtDebugBase();
    core_ctx.__GICCPUInterfaceBase = _reset___GICCPUInterfaceBase();
    core_ctx.__GICDistBase = _reset___GICDistBase();
    core_ctx.__GICITSControlBase = _reset___GICITSControlBase();
    core_ctx.__PMUBase = _reset___PMUBase();
    core_ctx.__syncAbortOnReadNormCache = true;
    core_ctx.__syncAbortOnReadNormNonCache = true;
    core_ctx.__syncAbortOnDeviceRead = true;
    core_ctx.__syncAbortOnSoRead = true;
    core_ctx.__syncAbortOnSoWrite = true;
    core_ctx.__syncAbortOnPrefetch = true;
    core_ctx.__syncAbortOnTTWCache = true;
    core_ctx.__syncAbortOnTTWNonCache = true;
    core_ctx.__syncAbortOnWriteNormCache = false;
    core_ctx.__syncAbortOnWriteNormNonCache = false;
    core_ctx.__syncAbortOnDeviceWrite = false;
    core_ctx.__unpred_tsize_aborts = true;
    core_ctx.__ignore_rvbar_in_aarch32 = false;
    core_ctx.__trickbox_enabled = false;
    core_ctx.__mops_forward_copy = true;
    core_ctx.__DBG_ROM_ADDR = _reset___DBG_ROM_ADDR();
    core_ctx.CFG_RMR_AA64 = _reset_CFG_RMR_AA64();
    core_ctx.ZCR_EL3_LEN_VALUE = _reset_ZCR_EL3_LEN_VALUE();
    core_ctx.CPTR_EL3_EZ_VALUE = _reset_CPTR_EL3_EZ_VALUE();
    core_ctx.CPTR_EL3_ESM_VALUE = _reset_CPTR_EL3_ESM_VALUE();
    core_ctx.SMCR_EL3_LEN_VALUE = _reset_SMCR_EL3_LEN_VALUE();
    core_ctx.__has_spe_pseudo_cycles = false;
    core_ctx.HEAP_BASE = _reset_HEAP_BASE();
    core_ctx.HEAP_LIMIT = _reset_HEAP_LIMIT();
    core_ctx.STACK_BASE = _reset_STACK_BASE();
    core_ctx.STACK_LIMIT = _reset_STACK_LIMIT();
    core_ctx.__emulator_termination_opcode = _reset___emulator_termination_opcode()
}

/// Initialize the FEAT_AA32EL0_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L351.
pub const fn _reset_FEAT_AA32EL0_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AA32EL1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L353.
pub const fn _reset_FEAT_AA32EL1_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_AA32EL2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L355.
pub const fn _reset_FEAT_AA32EL2_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_AA32EL3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L357.
pub const fn _reset_FEAT_AA32EL3_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_AA64EL0_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L359.
pub const fn _reset_FEAT_AA64EL0_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AA64EL1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L361.
pub const fn _reset_FEAT_AA64EL1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AA64EL2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L363.
pub const fn _reset_FEAT_AA64EL2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AA64EL3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L365.
pub const fn _reset_FEAT_AA64EL3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EL0_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L367.
pub const fn _reset_FEAT_EL0_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EL1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L369.
pub const fn _reset_FEAT_EL1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EL2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L371.
pub const fn _reset_FEAT_EL2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EL3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L373.
pub const fn _reset_FEAT_EL3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AES_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L375.
pub const fn _reset_FEAT_AES_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AdvSIMD_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L377.
pub const fn _reset_FEAT_AdvSIMD_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CSV2_1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L379.
pub const fn _reset_FEAT_CSV2_1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CSV2_1p2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L381.
pub const fn _reset_FEAT_CSV2_1p2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CSV2_2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L383.
pub const fn _reset_FEAT_CSV2_2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CSV2_3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L385.
pub const fn _reset_FEAT_CSV2_3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DoubleLock_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L387.
pub const fn _reset_FEAT_DoubleLock_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETMv4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L389.
pub const fn _reset_FEAT_ETMv4_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_ETMv4p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L391.
pub const fn _reset_FEAT_ETMv4p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETMv4p2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L393.
pub const fn _reset_FEAT_ETMv4p2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETMv4p3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L395.
pub const fn _reset_FEAT_ETMv4p3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETMv4p4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L397.
pub const fn _reset_FEAT_ETMv4p4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETMv4p5_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L399.
pub const fn _reset_FEAT_ETMv4p5_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETMv4p6_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L401.
pub const fn _reset_FEAT_ETMv4p6_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETS2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L403.
pub const fn _reset_FEAT_ETS2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FP_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L405.
pub const fn _reset_FEAT_FP_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GICv3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L407.
pub const fn _reset_FEAT_GICv3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GICv3_LEGACY_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L409.
pub const fn _reset_FEAT_GICv3_LEGACY_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GICv3_TDIR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L411.
pub const fn _reset_FEAT_GICv3_TDIR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GICv3p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L413.
pub const fn _reset_FEAT_GICv3p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GICv4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L415.
pub const fn _reset_FEAT_GICv4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GICv4p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L417.
pub const fn _reset_FEAT_GICv4p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_IVIPT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L419.
pub const fn _reset_FEAT_IVIPT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PCSRv8_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L421.
pub const fn _reset_FEAT_PCSRv8_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMULL_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L423.
pub const fn _reset_FEAT_PMULL_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L425.
pub const fn _reset_FEAT_PMUv3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_EXT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L427.
pub const fn _reset_FEAT_PMUv3_EXT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_EXT32_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L429.
pub const fn _reset_FEAT_PMUv3_EXT32_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SHA1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L431.
pub const fn _reset_FEAT_SHA1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SHA256_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L433.
pub const fn _reset_FEAT_SHA256_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TRC_EXT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L435.
pub const fn _reset_FEAT_TRC_EXT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TRC_SR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L437.
pub const fn _reset_FEAT_TRC_SR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_nTLBPA_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L439.
pub const fn _reset_FEAT_nTLBPA_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CRC32_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L441.
pub const fn _reset_FEAT_CRC32_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_Debugv8p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L443.
pub const fn _reset_FEAT_Debugv8p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_HAFDBS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L445.
pub const fn _reset_FEAT_HAFDBS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_HPDS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L447.
pub const fn _reset_FEAT_HPDS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LOR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L449.
pub const fn _reset_FEAT_LOR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LSE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L451.
pub const fn _reset_FEAT_LSE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PAN_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L453.
pub const fn _reset_FEAT_PAN_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L455.
pub const fn _reset_FEAT_PMUv3p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RDM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L457.
pub const fn _reset_FEAT_RDM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_VHE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L459.
pub const fn _reset_FEAT_VHE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_VMID16_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L461.
pub const fn _reset_FEAT_VMID16_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AA32BF16_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L463.
pub const fn _reset_FEAT_AA32BF16_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AA32HPD_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L465.
pub const fn _reset_FEAT_AA32HPD_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AA32I8MM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L467.
pub const fn _reset_FEAT_AA32I8MM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ASMv8p2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L469.
pub const fn _reset_FEAT_ASMv8p2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DPB_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L471.
pub const fn _reset_FEAT_DPB_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_Debugv8p2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L473.
pub const fn _reset_FEAT_Debugv8p2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EDHSR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L475.
pub const fn _reset_FEAT_EDHSR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_F32MM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L477.
pub const fn _reset_FEAT_F32MM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_F64MM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L479.
pub const fn _reset_FEAT_F64MM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FP16_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L481.
pub const fn _reset_FEAT_FP16_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_HPDS2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L483.
pub const fn _reset_FEAT_HPDS2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_I8MM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L485.
pub const fn _reset_FEAT_I8MM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_IESB_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L487.
pub const fn _reset_FEAT_IESB_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LPA_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L489.
pub const fn _reset_FEAT_LPA_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LSMAOC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L491.
pub const fn _reset_FEAT_LSMAOC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LVA_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L493.
pub const fn _reset_FEAT_LVA_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MPAM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L495.
pub const fn _reset_FEAT_MPAM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PAN2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L497.
pub const fn _reset_FEAT_PAN2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PCSRv8p2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L499.
pub const fn _reset_FEAT_PCSRv8p2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RAS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L501.
pub const fn _reset_FEAT_RAS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SHA3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L503.
pub const fn _reset_FEAT_SHA3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SHA512_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L505.
pub const fn _reset_FEAT_SHA512_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SM3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L507.
pub const fn _reset_FEAT_SM3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SM4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L509.
pub const fn _reset_FEAT_SM4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L511.
pub const fn _reset_FEAT_SPE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L513.
pub const fn _reset_FEAT_SVE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TTCNP_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L515.
pub const fn _reset_FEAT_TTCNP_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_UAO_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L517.
pub const fn _reset_FEAT_UAO_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_VPIPT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L519.
pub const fn _reset_FEAT_VPIPT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_XNX_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L521.
pub const fn _reset_FEAT_XNX_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CCIDX_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L523.
pub const fn _reset_FEAT_CCIDX_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CONSTPACFIELD_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L525.
pub const fn _reset_FEAT_CONSTPACFIELD_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_EPAC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L527.
pub const fn _reset_FEAT_EPAC_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_FCMA_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L529.
pub const fn _reset_FEAT_FCMA_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FPAC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L531.
pub const fn _reset_FEAT_FPAC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FPACCOMBINE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L533.
pub const fn _reset_FEAT_FPACCOMBINE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_JSCVT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L535.
pub const fn _reset_FEAT_JSCVT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LRCPC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L537.
pub const fn _reset_FEAT_LRCPC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_NV_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L539.
pub const fn _reset_FEAT_NV_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PACIMP_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L541.
pub const fn _reset_FEAT_PACIMP_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_PACQARMA3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L543.
pub const fn _reset_FEAT_PACQARMA3_IMPLEMENTED() -> bool {
    false
}

/// Initialize the FEAT_PACQARMA5_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L545.
pub const fn _reset_FEAT_PACQARMA5_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PAuth_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L547.
pub const fn _reset_FEAT_PAuth_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPEv1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L549.
pub const fn _reset_FEAT_SPEv1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AMUv1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L551.
pub const fn _reset_FEAT_AMUv1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_BBM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L553.
pub const fn _reset_FEAT_BBM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CNTSC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L555.
pub const fn _reset_FEAT_CNTSC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DIT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L557.
pub const fn _reset_FEAT_DIT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_Debugv8p4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L559.
pub const fn _reset_FEAT_Debugv8p4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DotProd_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L561.
pub const fn _reset_FEAT_DotProd_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DoubleFault_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L563.
pub const fn _reset_FEAT_DoubleFault_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FHM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L565.
pub const fn _reset_FEAT_FHM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FlagM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L567.
pub const fn _reset_FEAT_FlagM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_IDST_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L569.
pub const fn _reset_FEAT_IDST_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LRCPC2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L571.
pub const fn _reset_FEAT_LRCPC2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LSE2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L573.
pub const fn _reset_FEAT_LSE2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_NV2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L575.
pub const fn _reset_FEAT_NV2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3p4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L577.
pub const fn _reset_FEAT_PMUv3p4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RASSAv1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L579.
pub const fn _reset_FEAT_RASSAv1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RASv1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L581.
pub const fn _reset_FEAT_RASv1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_S2FWB_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L583.
pub const fn _reset_FEAT_S2FWB_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SEL2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L585.
pub const fn _reset_FEAT_SEL2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TLBIOS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L587.
pub const fn _reset_FEAT_TLBIOS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TLBIRANGE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L589.
pub const fn _reset_FEAT_TLBIRANGE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TRF_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L591.
pub const fn _reset_FEAT_TRF_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TTL_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L593.
pub const fn _reset_FEAT_TTL_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TTST_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L595.
pub const fn _reset_FEAT_TTST_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_BTI_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L597.
pub const fn _reset_FEAT_BTI_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CSV2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L599.
pub const fn _reset_FEAT_CSV2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CSV3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L601.
pub const fn _reset_FEAT_CSV3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DPB2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L603.
pub const fn _reset_FEAT_DPB2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_E0PD_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L605.
pub const fn _reset_FEAT_E0PD_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EVT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L607.
pub const fn _reset_FEAT_EVT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ExS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L609.
pub const fn _reset_FEAT_ExS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FRINTTS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L611.
pub const fn _reset_FEAT_FRINTTS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FlagM2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L613.
pub const fn _reset_FEAT_FlagM2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GTG_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L615.
pub const fn _reset_FEAT_GTG_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L617.
pub const fn _reset_FEAT_MTE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L619.
pub const fn _reset_FEAT_MTE2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3p5_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L621.
pub const fn _reset_FEAT_PMUv3p5_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RNG_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L623.
pub const fn _reset_FEAT_RNG_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RNG_TRAP_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L625.
pub const fn _reset_FEAT_RNG_TRAP_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SB_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L627.
pub const fn _reset_FEAT_SB_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPECRES_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L629.
pub const fn _reset_FEAT_SPECRES_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SSBS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L631.
pub const fn _reset_FEAT_SSBS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SSBS2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L633.
pub const fn _reset_FEAT_SSBS2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AMUv1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L635.
pub const fn _reset_FEAT_AMUv1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_BF16_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L637.
pub const fn _reset_FEAT_BF16_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DGH_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L639.
pub const fn _reset_FEAT_DGH_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ECV_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L641.
pub const fn _reset_FEAT_ECV_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FGT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L643.
pub const fn _reset_FEAT_FGT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_HPMN0_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L645.
pub const fn _reset_FEAT_HPMN0_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MPAMv0p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L647.
pub const fn _reset_FEAT_MPAMv0p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MPAMv1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L649.
pub const fn _reset_FEAT_MPAMv1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTPMU_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L651.
pub const fn _reset_FEAT_MTPMU_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PAuth2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L653.
pub const fn _reset_FEAT_PAuth2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TWED_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L655.
pub const fn _reset_FEAT_TWED_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AFP_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L657.
pub const fn _reset_FEAT_AFP_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EBF16_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L659.
pub const fn _reset_FEAT_EBF16_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_HCX_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L661.
pub const fn _reset_FEAT_HCX_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LPA2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L663.
pub const fn _reset_FEAT_LPA2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LS64_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L665.
pub const fn _reset_FEAT_LS64_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LS64_ACCDATA_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L667.
pub const fn _reset_FEAT_LS64_ACCDATA_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LS64_V_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L669.
pub const fn _reset_FEAT_LS64_V_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L671.
pub const fn _reset_FEAT_MTE3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PAN3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L673.
pub const fn _reset_FEAT_PAN3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3p7_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L675.
pub const fn _reset_FEAT_PMUv3p7_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RPRES_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L677.
pub const fn _reset_FEAT_RPRES_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPEv1p2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L679.
pub const fn _reset_FEAT_SPEv1p2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_WFxT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L681.
pub const fn _reset_FEAT_WFxT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_XS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L683.
pub const fn _reset_FEAT_XS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CMOW_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L685.
pub const fn _reset_FEAT_CMOW_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_Debugv8p8_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L687.
pub const fn _reset_FEAT_Debugv8p8_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GICv3_NMI_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L689.
pub const fn _reset_FEAT_GICv3_NMI_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_HBC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L691.
pub const fn _reset_FEAT_HBC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MOPS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L693.
pub const fn _reset_FEAT_MOPS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_NMI_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L695.
pub const fn _reset_FEAT_NMI_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_EXT64_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L697.
pub const fn _reset_FEAT_PMUv3_EXT64_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_TH_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L699.
pub const fn _reset_FEAT_PMUv3_TH_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3p8_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L701.
pub const fn _reset_FEAT_PMUv3p8_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SCTLR2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L703.
pub const fn _reset_FEAT_SCTLR2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPEv1p3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L705.
pub const fn _reset_FEAT_SPEv1p3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TCR2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L707.
pub const fn _reset_FEAT_TCR2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TIDCP1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L709.
pub const fn _reset_FEAT_TIDCP1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ADERR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L711.
pub const fn _reset_FEAT_ADERR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_AIE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L713.
pub const fn _reset_FEAT_AIE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ANERR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L715.
pub const fn _reset_FEAT_ANERR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CLRBHB_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L717.
pub const fn _reset_FEAT_CLRBHB_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CSSC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L719.
pub const fn _reset_FEAT_CSSC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_Debugv8p9_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L721.
pub const fn _reset_FEAT_Debugv8p9_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DoubleFault2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L723.
pub const fn _reset_FEAT_DoubleFault2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ECBHB_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L725.
pub const fn _reset_FEAT_ECBHB_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_FGT2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L727.
pub const fn _reset_FEAT_FGT2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_HAFT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L729.
pub const fn _reset_FEAT_HAFT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LRCPC3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L731.
pub const fn _reset_FEAT_LRCPC3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L733.
pub const fn _reset_FEAT_MTE4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_ASYM_FAULT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L735.
pub const fn _reset_FEAT_MTE_ASYM_FAULT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_ASYNC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L737.
pub const fn _reset_FEAT_MTE_ASYNC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L739.
pub const fn _reset_FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L741.
pub const fn _reset_FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_PERM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L743.
pub const fn _reset_FEAT_MTE_PERM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_STORE_ONLY_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L745.
pub const fn _reset_FEAT_MTE_STORE_ONLY_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MTE_TAGGED_FAR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L747.
pub const fn _reset_FEAT_MTE_TAGGED_FAR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PCSRv8p9_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L749.
pub const fn _reset_FEAT_PCSRv8p9_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PFAR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L751.
pub const fn _reset_FEAT_PFAR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_EDGE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L753.
pub const fn _reset_FEAT_PMUv3_EDGE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_ICNTR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L755.
pub const fn _reset_FEAT_PMUv3_ICNTR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3_SS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L757.
pub const fn _reset_FEAT_PMUv3_SS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PMUv3p9_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L759.
pub const fn _reset_FEAT_PMUv3p9_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_PRFMSLC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L761.
pub const fn _reset_FEAT_PRFMSLC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RASSAv2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L763.
pub const fn _reset_FEAT_RASSAv2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RASv2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L765.
pub const fn _reset_FEAT_RASv2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RPRFM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L767.
pub const fn _reset_FEAT_RPRFM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_S1PIE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L769.
pub const fn _reset_FEAT_S1PIE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_S1POE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L771.
pub const fn _reset_FEAT_S1POE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_S2PIE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L773.
pub const fn _reset_FEAT_S2PIE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_S2POE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L775.
pub const fn _reset_FEAT_S2POE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPECRES2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L777.
pub const fn _reset_FEAT_SPECRES2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPE_CRR_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L779.
pub const fn _reset_FEAT_SPE_CRR_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPE_FDS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L781.
pub const fn _reset_FEAT_SPE_FDS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPEv1p4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L783.
pub const fn _reset_FEAT_SPEv1p4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SPMU_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L785.
pub const fn _reset_FEAT_SPMU_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_THE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L787.
pub const fn _reset_FEAT_THE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_DoPD_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L789.
pub const fn _reset_FEAT_DoPD_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L791.
pub const fn _reset_FEAT_ETE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L793.
pub const fn _reset_FEAT_SVE2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE_AES_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L795.
pub const fn _reset_FEAT_SVE_AES_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE_BitPerm_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L797.
pub const fn _reset_FEAT_SVE_BitPerm_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE_PMULL128_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L799.
pub const fn _reset_FEAT_SVE_PMULL128_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE_SHA3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L801.
pub const fn _reset_FEAT_SVE_SHA3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE_SM4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L803.
pub const fn _reset_FEAT_SVE_SM4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TME_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L805.
pub const fn _reset_FEAT_TME_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TRBE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L807.
pub const fn _reset_FEAT_TRBE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETEv1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L809.
pub const fn _reset_FEAT_ETEv1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_BRBE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L811.
pub const fn _reset_FEAT_BRBE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETEv1p2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L813.
pub const fn _reset_FEAT_ETEv1p2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_RME_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L815.
pub const fn _reset_FEAT_RME_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SME_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L817.
pub const fn _reset_FEAT_SME_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SME_F64F64_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L819.
pub const fn _reset_FEAT_SME_F64F64_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SME_FA64_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L821.
pub const fn _reset_FEAT_SME_FA64_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SME_I16I64_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L823.
pub const fn _reset_FEAT_SME_I16I64_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_BRBEv1p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L825.
pub const fn _reset_FEAT_BRBEv1p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_MEC_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L827.
pub const fn _reset_FEAT_MEC_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SME2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L829.
pub const fn _reset_FEAT_SME2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ABLE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L831.
pub const fn _reset_FEAT_ABLE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_CHK_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L833.
pub const fn _reset_FEAT_CHK_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_D128_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L835.
pub const fn _reset_FEAT_D128_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_EBEP_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L837.
pub const fn _reset_FEAT_EBEP_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ETEv1p3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L839.
pub const fn _reset_FEAT_ETEv1p3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_GCS_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L841.
pub const fn _reset_FEAT_GCS_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_ITE_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L843.
pub const fn _reset_FEAT_ITE_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LSE128_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L845.
pub const fn _reset_FEAT_LSE128_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_LVA3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L847.
pub const fn _reset_FEAT_LVA3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SEBEP_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L849.
pub const fn _reset_FEAT_SEBEP_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SME2p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L851.
pub const fn _reset_FEAT_SME2p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SME_F16F16_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L853.
pub const fn _reset_FEAT_SME_F16F16_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE2p1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L855.
pub const fn _reset_FEAT_SVE2p1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SVE_B16B16_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L857.
pub const fn _reset_FEAT_SVE_B16B16_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SYSINSTR128_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L859.
pub const fn _reset_FEAT_SYSINSTR128_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_SYSREG128_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L861.
pub const fn _reset_FEAT_SYSREG128_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TRBE_EXT_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L863.
pub const fn _reset_FEAT_TRBE_EXT_IMPLEMENTED() -> bool {
    true
}

/// Initialize the FEAT_TRBE_MPAM_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L865.
pub const fn _reset_FEAT_TRBE_MPAM_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap0_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L867.
pub const fn _reset_v8Ap0_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L869.
pub const fn _reset_v8Ap1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L871.
pub const fn _reset_v8Ap2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L873.
pub const fn _reset_v8Ap3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L875.
pub const fn _reset_v8Ap4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap5_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L877.
pub const fn _reset_v8Ap5_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap6_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L879.
pub const fn _reset_v8Ap6_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap7_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L881.
pub const fn _reset_v8Ap7_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap8_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L883.
pub const fn _reset_v8Ap8_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v8Ap9_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L885.
pub const fn _reset_v8Ap9_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v9Ap0_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L887.
pub const fn _reset_v9Ap0_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v9Ap1_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L889.
pub const fn _reset_v9Ap1_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v9Ap2_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L891.
pub const fn _reset_v9Ap2_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v9Ap3_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L893.
pub const fn _reset_v9Ap3_IMPLEMENTED() -> bool {
    true
}

/// Initialize the v9Ap4_IMPLEMENTED register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L895.
pub const fn _reset_v9Ap4_IMPLEMENTED() -> bool {
    true
}

/// Initialize the NUM_AMU_COUNTER_GROUPS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1192.
pub const fn _reset_NUM_AMU_COUNTER_GROUPS() -> i128 {
    2
}

/// Initialize the NUM_AMU_CG0_MONITORS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1194.
pub const fn _reset_NUM_AMU_CG0_MONITORS() -> i128 {
    4
}

/// Initialize the NUM_AMU_CG1_MONITORS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1196.
pub const fn _reset_NUM_AMU_CG1_MONITORS() -> i128 {
    16
}

/// Initialize the NUM_PMU_COUNTERS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1198.
pub const fn _reset_NUM_PMU_COUNTERS() -> i128 {
    31
}

/// Initialize the NUM_BRBE_RECORDS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1200.
pub const fn _reset_NUM_BRBE_RECORDS() -> i128 {
    64
}

/// Initialize the NUM_GIC_PREEMPTION_BITS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1202.
pub const fn _reset_NUM_GIC_PREEMPTION_BITS() -> i128 {
    5
}

/// Initialize the NUM_GIC_PRIORITY_BITS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1204.
pub const fn _reset_NUM_GIC_PRIORITY_BITS() -> i128 {
    5
}

/// Initialize the NUM_GIC_LIST_REGS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1206.
pub const fn _reset_NUM_GIC_LIST_REGS() -> i128 {
    16
}

/// Initialize the NUM_BREAKPOINTS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1208.
pub fn _reset_NUM_BREAKPOINTS() -> i128 {
    -(1)
}

/// Initialize the NUM_WATCHPOINTS register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L1210.
pub fn _reset_NUM_WATCHPOINTS() -> i128 {
    -(1)
}

/// Initialize the __apply_effective_shareability register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3509.
pub const fn _reset___apply_effective_shareability() -> bool {
    true
}

/// Initialize the __cpy_mops_option_a_supported register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3511.
pub const fn _reset___cpy_mops_option_a_supported() -> bool {
    true
}

/// Initialize the __cpyf_mops_option_a_supported register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3513.
pub const fn _reset___cpyf_mops_option_a_supported() -> bool {
    true
}

/// Initialize the __empam_force_ns_RAO register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3515.
pub const fn _reset___empam_force_ns_RAO() -> bool {
    false
}

/// Initialize the __empam_force_ns_implemented register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3517.
pub const fn _reset___empam_force_ns_implemented() -> bool {
    false
}

/// Initialize the __empam_sdeflt_implemented register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3519.
pub const fn _reset___empam_sdeflt_implemented() -> bool {
    false
}

/// Initialize the __empam_tidr_implemented register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3521.
pub const fn _reset___empam_tidr_implemented() -> bool {
    false
}

/// Initialize the __feat_rpres register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3523.
pub const fn _reset___feat_rpres() -> bool {
    true
}

/// Initialize the __has_sme_priority_control register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3525.
pub const fn _reset___has_sme_priority_control() -> bool {
    true
}

/// Initialize the __isb_is_branch register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3527.
pub const fn _reset___isb_is_branch() -> bool {
    true
}

/// Initialize the __mpam_frac register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3529.
pub fn _reset___mpam_frac() -> BitVector<4> {
    CFG_MPAM_frac_none
}

/// Initialize the __mpam_major register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3531.
pub fn _reset___mpam_major() -> BitVector<4> {
    CFG_MPAM_none
}

/// Initialize the __mte_implemented register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3533.
pub fn _reset___mte_implemented() -> BitVector<4> {
    BitVector::<4>::new(0b0010)
}

/// Initialize the __set_mops_option_a_supported register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3535.
pub const fn _reset___set_mops_option_a_supported() -> bool {
    true
}

/// Initialize the __setg_mops_option_a_supported register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3537.
pub const fn _reset___setg_mops_option_a_supported() -> bool {
    true
}

/// Initialize the __sme_only register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3539.
pub const fn _reset___sme_only() -> bool {
    false
}

/// Initialize the __block_bbm_implemented register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3859.
pub fn _reset___block_bbm_implemented() -> i128 {
    UInt0(BitVector::<4>::new(0b0010))
}

/// Initialize the __has_sve_extended_bf16 register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3861.
pub const fn _reset___has_sve_extended_bf16() -> i128 {
    2
}

/// Initialize the __max_implemented_smeveclen register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3863.
pub const fn _reset___max_implemented_smeveclen() -> i128 {
    512
}

/// Initialize the __max_implemented_sveveclen register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3865.
pub const fn _reset___max_implemented_sveveclen() -> i128 {
    2048
}

/// Initialize the __supported_pa_size register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L3867.
pub const fn _reset___supported_pa_size() -> i128 {
    56
}

/// Initialize the CFG_RVBAR register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L5666.
pub fn _reset_CFG_RVBAR() -> BitVector<64> {
    ZeroExtend0(BitVector::<4>::new(0b0000), 64)
}

/// Initialize the __impdef_TG0 register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L5668.
pub fn _reset___impdef_TG0() -> BitVector<2> {
    BitVector::<2>::new(0b00)
}

/// Initialize the __impdef_TG1 register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L5670.
pub fn _reset___impdef_TG1() -> BitVector<2> {
    BitVector::<2>::new(0b10)
}

/// Initialize the __mpam_has_hcr register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L5672.
pub const fn _reset___mpam_has_hcr() -> bool {
    true
}

/// Initialize the __mpam_partid_max register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L5674.
pub fn _reset___mpam_partid_max() -> BitVector<16> {
    Zeros::<16>(16)
}

/// Initialize the __mpam_pmg_max register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L5676.
pub fn _reset___mpam_pmg_max() -> BitVector<8> {
    Zeros::<8>(8)
}

/// Initialize the __mpam_vpmr_max register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L5678.
pub fn _reset___mpam_vpmr_max() -> BitVector<3> {
    Zeros::<3>(3)
}

/// Initialize the __GIC_Active register.
///
/// Generated from the Sail sources at `src/devices.sail` L65.
pub fn _reset___GIC_Active() -> Option<InterruptID> {
    None
}

/// Initialize the __GIC_Pending register.
///
/// Generated from the Sail sources at `src/devices.sail` L64.
pub fn _reset___GIC_Pending() -> Option<InterruptID> {
    None
}

/// Initialize the __tlb_enabled register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L19790.
pub const fn _reset___tlb_enabled() -> bool {
    false
}

/// Initialize the __exclusive_granule_size register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39963.
pub fn _reset___exclusive_granule_size() -> BitVector<4> {
    BitVector::<4>::new(0b0100)
}

/// Initialize the __num_ctx_breakpoints register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39951.
pub fn _reset___num_ctx_breakpoints(core_ctx: &mut Core) -> i128 {
    core_ctx.NUM_BREAKPOINTS
}

/// Initialize the CFG_MPIDR register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39991.
pub fn _reset_CFG_MPIDR() -> BitVector<32> {
    BitVector::<32>::new(0b10000000000000000000000000000000)
}

/// Initialize the __CNTbase_frequency register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39959.
pub fn _reset___CNTbase_frequency() -> BitVector<32> {
    BitVector::<32>::new(0b00000101111101011110000100000000)
}

/// Initialize the __dczid_log2_block_size register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39947.
pub fn _reset___dczid_log2_block_size() -> i128 {
    UInt0(BitVector::<4>::new(0b1000))
}

/// Initialize the __gmid_log2_block_size register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39949.
pub fn _reset___gmid_log2_block_size() -> i128 {
    UInt0(BitVector::<4>::new(0b0100))
}

/// Initialize the __mecid_width register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39989.
pub fn _reset___mecid_width() -> BitVector<4> {
    BitVector::<4>::new(0b0000)
}

/// Initialize the __mpam_has_altsp register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39965.
pub const fn _reset___mpam_has_altsp() -> bool {
    true
}

/// Initialize the __rme_l0gptsz register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39977.
pub fn _reset___rme_l0gptsz() -> BitVector<4> {
    Zeros::<4>(4)
}

/// Initialize the __supported_va_size register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39943.
pub const fn _reset___supported_va_size() -> i128 {
    56
}

/// Initialize the __g1_activity_monitor_implemented register.
///
/// Generated from the Sail sources at `src/impdefs.sail` L867.
pub fn _reset___g1_activity_monitor_implemented() -> BitVector<16> {
    BitVector::<16>::new(0b0000000000000000)
}

/// Initialize the __g1_activity_monitor_offset_implemented register.
///
/// Generated from the Sail sources at `src/impdefs.sail` L874.
pub fn _reset___g1_activity_monitor_offset_implemented() -> BitVector<16> {
    BitVector::<16>::new(0b0000000000000000)
}

/// Initialize the __CTIBase register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39850.
pub fn _reset___CTIBase() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00100010000000100000000000000000)),
        55,
        0,
    )
}

/// Initialize the __CNTControlBase register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39852.
pub fn _reset___CNTControlBase() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00010110001000000000000000000000)),
        55,
        0,
    )
}

/// Initialize the __ExtDebugBase register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39854.
pub fn _reset___ExtDebugBase() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00100010000000010000000000000000)),
        55,
        0,
    )
}

/// Initialize the __GICCPUInterfaceBase register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39856.
pub fn _reset___GICCPUInterfaceBase() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00010011000010000010000000000000)),
        55,
        0,
    )
}

/// Initialize the __GICDistBase register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39858.
pub fn _reset___GICDistBase() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00101100000000010000000000000000)),
        55,
        0,
    )
}

/// Initialize the __GICITSControlBase register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39860.
pub fn _reset___GICITSControlBase() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00101100000100100000000000000000)),
        55,
        0,
    )
}

/// Initialize the __PMUBase register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39862.
pub fn _reset___PMUBase() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00100010000000110000000000000000)),
        55,
        0,
    )
}

/// Initialize the __syncAbortOnReadNormCache register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39882.
pub const fn _reset___syncAbortOnReadNormCache() -> bool {
    true
}

/// Initialize the __syncAbortOnReadNormNonCache register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39884.
pub const fn _reset___syncAbortOnReadNormNonCache() -> bool {
    true
}

/// Initialize the __syncAbortOnDeviceRead register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39886.
pub const fn _reset___syncAbortOnDeviceRead() -> bool {
    true
}

/// Initialize the __syncAbortOnSoRead register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39888.
pub const fn _reset___syncAbortOnSoRead() -> bool {
    true
}

/// Initialize the __syncAbortOnSoWrite register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39890.
pub const fn _reset___syncAbortOnSoWrite() -> bool {
    true
}

/// Initialize the __syncAbortOnPrefetch register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39892.
pub const fn _reset___syncAbortOnPrefetch() -> bool {
    true
}

/// Initialize the __syncAbortOnTTWCache register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39894.
pub const fn _reset___syncAbortOnTTWCache() -> bool {
    true
}

/// Initialize the __syncAbortOnTTWNonCache register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39896.
pub const fn _reset___syncAbortOnTTWNonCache() -> bool {
    true
}

/// Initialize the __syncAbortOnWriteNormCache register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39898.
pub const fn _reset___syncAbortOnWriteNormCache() -> bool {
    false
}

/// Initialize the __syncAbortOnWriteNormNonCache register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39900.
pub const fn _reset___syncAbortOnWriteNormNonCache() -> bool {
    false
}

/// Initialize the __syncAbortOnDeviceWrite register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39902.
pub const fn _reset___syncAbortOnDeviceWrite() -> bool {
    false
}

/// Initialize the __unpred_tsize_aborts register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39945.
pub const fn _reset___unpred_tsize_aborts() -> bool {
    true
}

/// Initialize the __ignore_rvbar_in_aarch32 register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39953.
pub const fn _reset___ignore_rvbar_in_aarch32() -> bool {
    false
}

/// Initialize the __trickbox_enabled register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39955.
pub const fn _reset___trickbox_enabled() -> bool {
    false
}

/// Initialize the __mops_forward_copy register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39957.
pub const fn _reset___mops_forward_copy() -> bool {
    true
}

/// Initialize the __DBG_ROM_ADDR register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39961.
pub fn _reset___DBG_ROM_ADDR() -> BitVector<56> {
    integer_subrange(
        UInt0(BitVector::<32>::new(0b00100010000000000000000000000000)),
        55,
        0,
    )
}

/// Initialize the CFG_RMR_AA64 register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39967.
pub fn _reset_CFG_RMR_AA64() -> BitVector<1> {
    BitVector::<1>::new(0b1)
}

/// Initialize the ZCR_EL3_LEN_VALUE register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39969.
pub fn _reset_ZCR_EL3_LEN_VALUE() -> i128 {
    -(1)
}

/// Initialize the CPTR_EL3_EZ_VALUE register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39971.
pub fn _reset_CPTR_EL3_EZ_VALUE() -> i128 {
    -(1)
}

/// Initialize the CPTR_EL3_ESM_VALUE register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39973.
pub fn _reset_CPTR_EL3_ESM_VALUE() -> i128 {
    -(1)
}

/// Initialize the SMCR_EL3_LEN_VALUE register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39975.
pub fn _reset_SMCR_EL3_LEN_VALUE() -> i128 {
    -(1)
}

/// Initialize the __has_spe_pseudo_cycles register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39979.
pub const fn _reset___has_spe_pseudo_cycles() -> bool {
    false
}

/// Initialize the HEAP_BASE register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39981.
pub fn _reset_HEAP_BASE() -> BitVector<64> {
    ZeroExtend0(BitVector::<32>::new(0b00000000000000000000000000000000), 64)
}

/// Initialize the HEAP_LIMIT register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39983.
pub fn _reset_HEAP_LIMIT() -> BitVector<64> {
    ZeroExtend0(BitVector::<32>::new(0b00001111000000000000000000000000), 64)
}

/// Initialize the STACK_BASE register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39985.
pub fn _reset_STACK_BASE() -> BitVector<64> {
    ZeroExtend0(BitVector::<32>::new(0b00010000000000000000000000000000), 64)
}

/// Initialize the STACK_LIMIT register.
///
/// Generated from the Sail sources at `src/v8_base.sail` L39987.
pub fn _reset_STACK_LIMIT() -> BitVector<64> {
    ZeroExtend0(BitVector::<32>::new(0b00001111000000000000000000000000), 64)
}

/// Initialize the __emulator_termination_opcode register.
///
/// Generated from the Sail sources at `src/fetch.sail` L265.
pub fn _reset___emulator_termination_opcode() -> Option<BitVector<32>> {
    None
}

/// __id
///
/// Generated from the Sail sources at `sail/lib/flow.sail` L107.
pub fn __id(x: i128) -> i128 {
    x
}

/// result
///
/// Generated from the Sail sources at `sail/lib/result.sail` L60-63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum result<A, B> {
    Ok(A),
    Err(B),
}

/// Access_variety
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L57-61.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_variety {
    AV_plain,
    AV_exclusive,
    AV_atomic_rmw,
}

/// Access_strength
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L66-70.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_strength {
    AS_normal,
    AS_rel_or_acq,
    AS_acq_rcpc,
}

/// Explicit_access_kind
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L75-78.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Explicit_access_kind {
    pub variety: Access_variety,
    pub strength: Access_strength,
}

/// Access_kind
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L83-88.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_kind<ARCH_AK> {
    AK_explicit(Explicit_access_kind),
    AK_ifetch(()),
    AK_ttw(()),
    AK_arch(ARCH_AK),
}

/// Mem_read_request
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L93-104.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mem_read_request<const N: i128, const VASIZE: i128, PA, TS, ARCH_AK> {
    pub access_kind: Access_kind<ARCH_AK>,
    pub va: Option<BitVector<VASIZE>>,
    pub pa: PA,
    pub translation: TS,
    pub size: i128,
    pub tag: bool,
}

pub const __monomorphize_reads: bool = false;

pub const __monomorphize_writes: bool = false;

/// Zeros
///
/// Generated from the Sail sources at `src/prelude.sail` L159.
pub const fn Zeros<const N: i128>(n: i128) -> BitVector<N> {
    sail_zeros(n)
}

/// IsZero
///
/// Generated from the Sail sources at `src/prelude.sail` L162.
pub fn IsZero<const N: i128>(x: BitVector<N>) -> bool {
    (x == sail_zeros(bitvector_length(x)))
}

/// signal
///
/// Generated from the Sail sources at `src/prelude.sail` L258.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum signal {
    LOW,
    HIGH,
}

/// exception
///
/// Generated from the Sail sources at `src/prelude.sail` L266-275.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_Undefined(()),
    Error_See(&'static str),
    Error_ImplementationDefined(&'static str),
    Error_ReservedEncoding(()),
    Error_ExceptionTaken(()),
    Error_Unpredictable(()),
    Error_ConstrainedUnpredictable(()),
    Error_SError(bool),
}

pub type vector_length = i128;

pub type predicate_length = i128;

/// Configuration_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L41-54.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Configuration_Type {
    pub bits: BitVector<32>,
}

/// ACCDATA_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44131.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ACCDATA_EL1_Type {
    pub bits: BitVector<64>,
}

/// AMCFGR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46779-46780.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCFGR_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCFGR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52148-52149.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCFGR_Type {
    pub bits: BitVector<32>,
}

/// AMCG1IDR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46792-46826.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCG1IDR_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCGCR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46834.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCGCR_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCGCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52127.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCGCR_Type {
    pub bits: BitVector<32>,
}

/// AMCIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53409.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCIDR0_Type {
    pub bits: BitVector<32>,
}

/// AMCIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53417.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCIDR1_Type {
    pub bits: BitVector<32>,
}

/// AMCIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53405.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCIDR2_Type {
    pub bits: BitVector<32>,
}

/// AMCIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53392.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCIDR3_Type {
    pub bits: BitVector<32>,
}

/// AMCNTENCLR0_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46703-46704.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENCLR0_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCNTENCLR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52231-52232.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENCLR0_Type {
    pub bits: BitVector<32>,
}

/// AMCNTENCLR1_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46734-46752.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENCLR1_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCNTENCLR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52170-52188.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENCLR1_Type {
    pub bits: BitVector<32>,
}

/// AMCNTENSET0_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46698-46699.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENSET0_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCNTENSET0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52209-52210.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENSET0_Type {
    pub bits: BitVector<32>,
}

/// AMCNTENSET1_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46712-46730.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENSET1_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCNTENSET1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52253-52271.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCNTENSET1_Type {
    pub bits: BitVector<32>,
}

/// AMCR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46694.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCR_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52292.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMCR_Type {
    pub bits: BitVector<32>,
}

/// AMDEVARCH_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53396-53397.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMDEVARCH_Type {
    pub bits: BitVector<32>,
}

/// AMDEVTYPE_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53421.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMDEVTYPE_Type {
    pub bits: BitVector<32>,
}

/// AMEVCNTR0_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46756.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVCNTR0_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMEVCNTR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46668.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVCNTR0_Type {
    pub bits: BitVector<64>,
}

/// AMEVCNTR1_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46660.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVCNTR1_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMEVCNTR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46672.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVCNTR1_Type {
    pub bits: BitVector<64>,
}

/// AMEVTYPER0_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46708.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVTYPER0_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMEVTYPER0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52072.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVTYPER0_Type {
    pub bits: BitVector<32>,
}

/// AMEVTYPER1_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46830.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVTYPER1_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMEVTYPER1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52105.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMEVTYPER1_Type {
    pub bits: BitVector<32>,
}

/// AMIIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53383-53384.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMIIDR_Type {
    pub bits: BitVector<32>,
}

/// AMPIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53401.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMPIDR0_Type {
    pub bits: BitVector<32>,
}

/// AMPIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53413.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMPIDR1_Type {
    pub bits: BitVector<32>,
}

/// AMPIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53375.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMPIDR2_Type {
    pub bits: BitVector<32>,
}

/// AMPIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53388.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMPIDR3_Type {
    pub bits: BitVector<32>,
}

/// AMPIDR4_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53379.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMPIDR4_Type {
    pub bits: BitVector<32>,
}

/// AMUSERENR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46788.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMUSERENR_EL0_Type {
    pub bits: BitVector<64>,
}

/// AMUSERENR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52084.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AMUSERENR_Type {
    pub bits: BitVector<32>,
}

/// BRBCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5487-5498.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5502-5513.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// BRBFCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5570-5582.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBFCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBIDR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6418-6419.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBIDR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBINFINJ_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L35616-35626.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBINFINJ_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBINFType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1368-1378.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBINFType {
    pub bits: BitVector<64>,
}

/// BRBINF_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47052-47062.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBINF_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBSRCINJ_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L35630.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBSRCINJ_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBSRCType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1364.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBSRCType {
    pub bits: BitVector<64>,
}

/// BRBSRC_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47070.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBSRC_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBTGTINJ_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L35634.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBTGTINJ_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBTGTType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1366.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBTGTType {
    pub bits: BitVector<64>,
}

/// BRBTGT_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47066.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBTGT_EL1_Type {
    pub bits: BitVector<64>,
}

/// BRBTS_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5586.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BRBTS_EL1_Type {
    pub bits: BitVector<64>,
}

/// CCSIDR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45512.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CCSIDR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// CCSIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50848.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CCSIDR2_Type {
    pub bits: BitVector<32>,
}

/// CCSIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45855.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CCSIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// CCSIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51813-51814.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CCSIDR_Type {
    pub bits: BitVector<32>,
}

/// CLIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12466-12486.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CLIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// CLIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47688-47701.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CLIDR_Type {
    pub bits: BitVector<32>,
}

/// CNTCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18319-18320.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTCR_Type {
    pub bits: BitVector<32>,
}

/// CNTEL0ACR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52970-52971.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTEL0ACR_Type {
    pub bits: BitVector<32>,
}

/// CNTFID0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52987.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTFID0_Type {
    pub bits: BitVector<32>,
}

/// CNTHCTL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5604-5623.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHCTL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHCTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48933-48941.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHCTL_Type {
    pub bits: BitVector<32>,
}

/// CNTHPS_CTL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18228-18229.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHPS_CTL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHPS_CTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47844-47845.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHPS_CTL_Type {
    pub bits: BitVector<32>,
}

/// CNTHPS_CVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18233.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHPS_CVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHPS_TVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45600.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHPS_TVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHP_CTL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18041-18042.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHP_CTL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHP_CTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18046-18047.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHP_CTL_Type {
    pub bits: BitVector<32>,
}

/// CNTHP_CVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18068.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHP_CVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHP_CVAL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18072.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHP_CVAL_Type {
    pub bits: BitVector<64>,
}

/// CNTHP_TVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45194.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHP_TVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHVS_CTL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18237-18238.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHVS_CTL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHVS_CTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49754-49755.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHVS_CTL_Type {
    pub bits: BitVector<32>,
}

/// CNTHVS_CVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18242.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHVS_CVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHVS_TVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45190.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHVS_TVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHV_CTL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18246-18247.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHV_CTL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHV_CTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50869-50870.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHV_CTL_Type {
    pub bits: BitVector<32>,
}

/// CNTHV_CVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18251.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHV_CVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTHV_TVAL_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44803.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTHV_TVAL_EL2_Type {
    pub bits: BitVector<64>,
}

/// CNTID_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53286.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTID_Type {
    pub bits: BitVector<32>,
}

/// CNTKCTL_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18379-18389.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTKCTL_EL1_Type {
    pub bits: BitVector<64>,
}

/// CNTKCTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L22448-22458.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTKCTL_Type {
    pub bits: BitVector<32>,
}

/// CNTNSAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53344-53354.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTNSAR_Type {
    pub bits: BitVector<32>,
}

/// CNTPS_CTL_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18255-18256.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTPS_CTL_EL1_Type {
    pub bits: BitVector<64>,
}

/// CNTPS_CVAL_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18260.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTPS_CVAL_EL1_Type {
    pub bits: BitVector<64>,
}

/// CNTPS_TVAL_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46162.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTPS_TVAL_EL1_Type {
    pub bits: BitVector<64>,
}

/// CNTP_CTL_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18084-18085.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTP_CTL_EL0_Type {
    pub bits: BitVector<64>,
}

/// CNTP_CTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18089-18090.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTP_CTL_Type {
    pub bits: BitVector<32>,
}

/// CNTP_CVAL_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18137.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTP_CVAL_EL0_Type {
    pub bits: BitVector<64>,
}

/// CNTP_CVAL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18141.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTP_CVAL_Type {
    pub bits: BitVector<64>,
}

/// CNTP_TVAL_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44178.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTP_TVAL_EL0_Type {
    pub bits: BitVector<64>,
}

/// CNTSCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18324.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTSCR_Type {
    pub bits: BitVector<32>,
}

/// CNTSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52763.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTSR_Type {
    pub bits: BitVector<32>,
}

/// CNTV_CTL_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18167-18168.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTV_CTL_EL0_Type {
    pub bits: BitVector<64>,
}

/// CNTV_CTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51585-51586.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTV_CTL_Type {
    pub bits: BitVector<32>,
}

/// CNTV_CVAL_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L18172.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTV_CVAL_EL0_Type {
    pub bits: BitVector<64>,
}

/// CNTV_CVAL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46495.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTV_CVAL_Type {
    pub bits: BitVector<64>,
}

/// CNTV_TVAL_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46178.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CNTV_TVAL_EL0_Type {
    pub bits: BitVector<64>,
}

/// CONTEXTIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12841.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CONTEXTIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// CONTEXTIDR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23113.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CONTEXTIDR_EL2_Type {
    pub bits: BitVector<64>,
}

/// CONTEXTIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12895.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CONTEXTIDR_Type {
    pub bits: BitVector<32>,
}

/// CPACR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7036-7037.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CPACR_EL1_Type {
    pub bits: BitVector<64>,
}

/// CPACR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7125-7126.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CPACR_Type {
    pub bits: BitVector<32>,
}

/// CPTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7041-7052.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CPTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// CPTR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7056-7064.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CPTR_EL3_Type {
    pub bits: BitVector<64>,
}

/// CSSELR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44821.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CSSELR_EL1_Type {
    pub bits: BitVector<64>,
}

/// CSSELR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48434.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CSSELR_Type {
    pub bits: BitVector<32>,
}

/// CTIAUTHSTATUS_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53299.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIAUTHSTATUS_Type {
    pub bits: BitVector<32>,
}

/// CTICIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53011.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTICIDR0_Type {
    pub bits: BitVector<32>,
}

/// CTICIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52706.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTICIDR1_Type {
    pub bits: BitVector<32>,
}

/// CTICIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53191.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTICIDR2_Type {
    pub bits: BitVector<32>,
}

/// CTICIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52936.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTICIDR3_Type {
    pub bits: BitVector<32>,
}

/// CTICONTROL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53073.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTICONTROL_Type {
    pub bits: BitVector<32>,
}

/// CTIDEVARCH_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52927-52928.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIDEVARCH_Type {
    pub bits: BitVector<32>,
}

/// CTIDEVCTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39677.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIDEVCTL_Type {
    pub bits: BitVector<32>,
}

/// CTIDEVID_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53322-53323.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIDEVID_Type {
    pub bits: BitVector<32>,
}

/// CTIDEVTYPE_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52724.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIDEVTYPE_Type {
    pub bits: BitVector<32>,
}

/// CTIITCTRL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53023.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIITCTRL_Type {
    pub bits: BitVector<32>,
}

/// CTILAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52697.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTILAR_Type {
    pub bits: BitVector<32>,
}

/// CTILSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39384.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTILSR_Type {
    pub bits: BitVector<32>,
}

/// CTIPIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52788.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIPIDR0_Type {
    pub bits: BitVector<32>,
}

/// CTIPIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52755.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIPIDR1_Type {
    pub bits: BitVector<32>,
}

/// CTIPIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52701-52702.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIPIDR2_Type {
    pub bits: BitVector<32>,
}

/// CTIPIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53052.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIPIDR3_Type {
    pub bits: BitVector<32>,
}

/// CTIPIDR4_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53265.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTIPIDR4_Type {
    pub bits: BitVector<32>,
}

/// CTR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L3845-3855.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTR_EL0_Type {
    pub bits: BitVector<64>,
}

/// CTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47792-47801.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CTR_Type {
    pub bits: BitVector<32>,
}

/// DACR32_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L38320-38338.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DACR32_EL2_Type {
    pub bits: BitVector<64>,
}

/// DACR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L38342-38360.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DACR_Type {
    pub bits: BitVector<32>,
}

/// DBGAUTHSTATUS_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46001-46011.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGAUTHSTATUS_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGAUTHSTATUS_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47930-47931.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGAUTHSTATUS_Type {
    pub bits: BitVector<32>,
}

/// DBGBCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23117-23130.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGBCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGBCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L40208-40217.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGBCR_Type {
    pub bits: BitVector<32>,
}

/// DBGBVR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23134-23141.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGBVR_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGBVR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L40229.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGBVR_Type {
    pub bits: BitVector<32>,
}

/// DBGBXVR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L40241.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGBXVR_Type {
    pub bits: BitVector<32>,
}

/// DBGCLAIMCLR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46202.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGCLAIMCLR_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGCLAIMCLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48203.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGCLAIMCLR_Type {
    pub bits: BitVector<32>,
}

/// DBGCLAIMSET_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44542.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGCLAIMSET_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGCLAIMSET_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49358.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGCLAIMSET_Type {
    pub bits: BitVector<32>,
}

/// DBGDCCINT_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39457.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDCCINT_Type {
    pub bits: BitVector<32>,
}

/// DBGDEVID1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48552.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDEVID1_Type {
    pub bits: BitVector<32>,
}

/// DBGDEVID_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50793-50803.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDEVID_Type {
    pub bits: BitVector<32>,
}

/// DBGDIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51334-51342.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDIDR_Type {
    pub bits: BitVector<32>,
}

/// DBGDRAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46540.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDRAR_Type {
    pub bits: BitVector<64>,
}

/// DBGDSCRext_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14414-14432.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDSCRext_Type {
    pub bits: BitVector<32>,
}

/// DBGDSCRint_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14434-14444.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDSCRint_Type {
    pub bits: BitVector<32>,
}

/// DBGDTRRXext_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49663.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDTRRXext_Type {
    pub bits: BitVector<32>,
}

/// DBGDTRRXint_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51464.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDTRRXint_Type {
    pub bits: BitVector<32>,
}

/// DBGDTRTXext_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49485.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDTRTXext_Type {
    pub bits: BitVector<32>,
}

/// DBGDTRTXint_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51735.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDTRTXint_Type {
    pub bits: BitVector<32>,
}

/// DBGDTR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39404.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGDTR_EL0_Type {
    pub bits: BitVector<64>,
}

/// DBGOSDLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7912.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGOSDLR_Type {
    pub bits: BitVector<32>,
}

/// DBGOSECCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50827.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGOSECCR_Type {
    pub bits: BitVector<32>,
}

/// DBGOSLAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49659.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGOSLAR_Type {
    pub bits: BitVector<32>,
}

/// DBGOSLSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8354-8355.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGOSLSR_Type {
    pub bits: BitVector<32>,
}

/// DBGPRCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7928.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGPRCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGPRCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7932.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGPRCR_Type {
    pub bits: BitVector<32>,
}

/// DBGVCR32_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L40509-40529.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGVCR32_EL2_Type {
    pub bits: BitVector<64>,
}

/// DBGVCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L40533-40558.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGVCR_Type {
    pub bits: BitVector<32>,
}

/// DBGWCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23682-23695.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGWCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGWCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L40629-40640.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGWCR_Type {
    pub bits: BitVector<32>,
}

/// DBGWVR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23699-23700.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGWVR_EL1_Type {
    pub bits: BitVector<64>,
}

/// DBGWVR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L40652.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DBGWVR_Type {
    pub bits: BitVector<32>,
}

/// DCZID_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L36716.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DCZID_EL0_Type {
    pub bits: BitVector<64>,
}

/// DFSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14299-14310.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DFSR_Type {
    pub bits: BitVector<32>,
}

/// DISR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33202-33203.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DISR_EL1_Type {
    pub bits: BitVector<64>,
}

/// DISR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33207-33216.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DISR_Type {
    pub bits: BitVector<32>,
}

/// DSPSR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5807.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DSPSR2_Type {
    pub bits: BitVector<32>,
}

/// DSPSR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5744-5770.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DSPSR_EL0_Type {
    pub bits: BitVector<64>,
}

/// DSPSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5774-5794.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DSPSR_Type {
    pub bits: BitVector<32>,
}

/// DormantCtl_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L56-70.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DormantCtl_Type {
    pub bits: BitVector<32>,
}

/// EDAA32PFR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47300-47301.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDAA32PFR_Type {
    pub bits: BitVector<64>,
}

/// EDCIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53212.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDCIDR0_Type {
    pub bits: BitVector<32>,
}

/// EDCIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53367.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDCIDR1_Type {
    pub bits: BitVector<32>,
}

/// EDCIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53278.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDCIDR2_Type {
    pub bits: BitVector<32>,
}

/// EDCIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52767.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDCIDR3_Type {
    pub bits: BitVector<32>,
}

/// EDDEVARCH_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52744-52751.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDDEVARCH_Type {
    pub bits: BitVector<32>,
}

/// EDDEVID1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53135.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDDEVID1_Type {
    pub bits: BitVector<32>,
}

/// EDDEVID_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53203-53204.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDDEVID_Type {
    pub bits: BitVector<32>,
}

/// EDDEVTYPE_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52831.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDDEVTYPE_Type {
    pub bits: BitVector<32>,
}

/// EDDFR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47260-47272.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDDFR1_Type {
    pub bits: BitVector<64>,
}

/// EDDFR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47280-47291.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDDFR_Type {
    pub bits: BitVector<64>,
}

/// EDECCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L10151-10175.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDECCR_Type {
    pub bits: BitVector<32>,
}

/// EDECR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5720-5721.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDECR_Type {
    pub bits: BitVector<32>,
}

/// EDESR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L10191.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDESR_Type {
    pub bits: BitVector<32>,
}

/// EDHSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47193-47204.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDHSR_Type {
    pub bits: BitVector<64>,
}

/// EDITCTRL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53358.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDITCTRL_Type {
    pub bits: BitVector<32>,
}

/// EDLAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53245.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDLAR_Type {
    pub bits: BitVector<32>,
}

/// EDLSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39388.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDLSR_Type {
    pub bits: BitVector<32>,
}

/// EDPCSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47208.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPCSR_Type {
    pub bits: BitVector<64>,
}

/// EDPFR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47169-47181.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPFR_Type {
    pub bits: BitVector<64>,
}

/// EDPIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53295.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPIDR0_Type {
    pub bits: BitVector<32>,
}

/// EDPIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52898.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPIDR1_Type {
    pub bits: BitVector<32>,
}

/// EDPIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53340.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPIDR2_Type {
    pub bits: BitVector<32>,
}

/// EDPIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52804.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPIDR3_Type {
    pub bits: BitVector<32>,
}

/// EDPIDR4_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53069.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPIDR4_Type {
    pub bits: BitVector<32>,
}

/// EDPRCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53139-53140.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPRCR_Type {
    pub bits: BitVector<32>,
}

/// EDPRSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39433-39449.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDPRSR_Type {
    pub bits: BitVector<32>,
}

/// EDRCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53195.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDRCR_Type {
    pub bits: BitVector<32>,
}

/// EDSCR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23157.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDSCR2_Type {
    pub bits: BitVector<32>,
}

/// EDSCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4191-4214.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDSCR_Type {
    pub bits: BitVector<32>,
}

/// EDVIDSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39742-39750.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct EDVIDSR_Type {
    pub bits: BitVector<32>,
}

/// ERRIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46938.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ERRIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ERRIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52313.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ERRIDR_Type {
    pub bits: BitVector<32>,
}

/// ERRSELR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46950.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ERRSELR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ERRSELR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52442.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ERRSELR_Type {
    pub bits: BitVector<32>,
}

/// ERRnFR_ElemType
///
/// Generated from the Sail sources at `src/v8_base.sail` L3675-3696.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ERRnFR_ElemType {
    pub bits: BitVector<64>,
}

/// ERXGSR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46970-47036.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ERXGSR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ESRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1338.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ESRType {
    pub bits: BitVector<64>,
}

/// ESR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9191-9192.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ESR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ESR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9196-9197.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ESR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ESR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9201-9202.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ESR_EL3_Type {
    pub bits: BitVector<64>,
}

/// FPCRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1340-1362.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FPCRType {
    pub bits: BitVector<64>,
}

/// FPCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L2743-2762.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FPCR_Type {
    pub bits: BitVector<64>,
}

/// FPEXC32_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L15458-15473.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FPEXC32_EL2_Type {
    pub bits: BitVector<64>,
}

/// FPEXC_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L15477-15492.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FPEXC_Type {
    pub bits: BitVector<32>,
}

/// FPSCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L3595-3621.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FPSCR_Type {
    pub bits: BitVector<32>,
}

/// FPSID_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49245-49253.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FPSID_Type {
    pub bits: BitVector<32>,
}

/// FPSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L2766-2779.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FPSR_Type {
    pub bits: BitVector<64>,
}

/// GCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L22678.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// GCSCRE0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L10936-10937.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSCRE0_EL1_Type {
    pub bits: BitVector<64>,
}

/// GCSCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8198-8199.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// GCSCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8203-8204.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// GCSCR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8208-8209.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSCR_EL3_Type {
    pub bits: BitVector<64>,
}

/// GCSPR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7014.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSPR_EL0_Type {
    pub bits: BitVector<64>,
}

/// GCSPR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7018.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSPR_EL1_Type {
    pub bits: BitVector<64>,
}

/// GCSPR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7022.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSPR_EL2_Type {
    pub bits: BitVector<64>,
}

/// GCSPR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7026.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GCSPR_EL3_Type {
    pub bits: BitVector<64>,
}

/// GICC_ABPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53261.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_ABPR_Type {
    pub bits: BitVector<32>,
}

/// GICC_AEOIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53249.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_AEOIR_Type {
    pub bits: BitVector<32>,
}

/// GICC_AHPPIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53253.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_AHPPIR_Type {
    pub bits: BitVector<32>,
}

/// GICC_AIAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53371.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_AIAR_Type {
    pub bits: BitVector<32>,
}

/// GICC_BPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53318.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_BPR_Type {
    pub bits: BitVector<32>,
}

/// GICC_CTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53056-53065.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_CTLR_Type {
    pub bits: BitVector<32>,
}

/// GICC_DIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53208.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_DIR_Type {
    pub bits: BitVector<32>,
}

/// GICC_EOIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53336.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_EOIR_Type {
    pub bits: BitVector<32>,
}

/// GICC_HPPIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53015.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_HPPIR_Type {
    pub bits: BitVector<32>,
}

/// GICC_IAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52693.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_IAR_Type {
    pub bits: BitVector<32>,
}

/// GICC_PMR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52919.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_PMR_Type {
    pub bits: BitVector<32>,
}

/// GICC_RPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52940.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_RPR_Type {
    pub bits: BitVector<32>,
}

/// GICC_STATUSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52808-52809.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICC_STATUSR_Type {
    pub bits: BitVector<32>,
}

/// GICD_CLRSPI_NSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53019.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_CLRSPI_NSR_Type {
    pub bits: BitVector<32>,
}

/// GICD_CLRSPI_SR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53115.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_CLRSPI_SR_Type {
    pub bits: BitVector<32>,
}

/// GICD_CTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52677-52689.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_CTLR_Type {
    pub bits: BitVector<32>,
}

/// GICD_IIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53362-53363.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_IIDR_Type {
    pub bits: BitVector<32>,
}

/// GICD_SETSPI_NSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53282.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_SETSPI_NSR_Type {
    pub bits: BitVector<32>,
}

/// GICD_SETSPI_SR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53327.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_SETSPI_SR_Type {
    pub bits: BitVector<32>,
}

/// GICD_SGIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53220-53226.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_SGIR_Type {
    pub bits: BitVector<32>,
}

/// GICD_STATUSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53148-53149.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_STATUSR_Type {
    pub bits: BitVector<32>,
}

/// GICD_TYPER2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52840-52841.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_TYPER2_Type {
    pub bits: BitVector<32>,
}

/// GICD_TYPER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52991-53007.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICD_TYPER_Type {
    pub bits: BitVector<32>,
}

/// GICH_EISR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52845-52863.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICH_EISR_Type {
    pub bits: BitVector<32>,
}

/// GICH_ELRSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52867-52885.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICH_ELRSR_Type {
    pub bits: BitVector<32>,
}

/// GICH_HCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53303-53314.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICH_HCR_Type {
    pub bits: BitVector<32>,
}

/// GICH_MISR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52710-52720.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICH_MISR_Type {
    pub bits: BitVector<32>,
}

/// GICH_VMCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53230-53241.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICH_VMCR_Type {
    pub bits: BitVector<32>,
}

/// GICH_VTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53040-53048.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICH_VTR_Type {
    pub bits: BitVector<32>,
}

/// GICM_CLRSPI_NSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52889.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICM_CLRSPI_NSR_Type {
    pub bits: BitVector<32>,
}

/// GICM_CLRSPI_SR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52902.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICM_CLRSPI_SR_Type {
    pub bits: BitVector<32>,
}

/// GICM_IIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52813-52814.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICM_IIDR_Type {
    pub bits: BitVector<32>,
}

/// GICM_SETSPI_NSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52771.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICM_SETSPI_NSR_Type {
    pub bits: BitVector<32>,
}

/// GICM_SETSPI_SR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52792.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICM_SETSPI_SR_Type {
    pub bits: BitVector<32>,
}

/// GICM_TYPER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52835-52836.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICM_TYPER_Type {
    pub bits: BitVector<32>,
}

/// GICR_CLRLPIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47185.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_CLRLPIR_Type {
    pub bits: BitVector<64>,
}

/// GICR_CTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52956-52966.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_CTLR_Type {
    pub bits: BitVector<32>,
}

/// GICR_IIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52739-52740.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_IIDR_Type {
    pub bits: BitVector<32>,
}

/// GICR_INMIR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53077-53111.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_INMIR0_Type {
    pub bits: BitVector<32>,
}

/// GICR_INVALLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47189.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_INVALLR_Type {
    pub bits: BitVector<64>,
}

/// GICR_INVLPIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47295-47296.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_INVLPIR_Type {
    pub bits: BitVector<64>,
}

/// GICR_ISENABLER0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53153-53187.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_ISENABLER0_Type {
    pub bits: BitVector<32>,
}

/// GICR_MPAMIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52932.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_MPAMIDR_Type {
    pub bits: BitVector<32>,
}

/// GICR_PARTIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53131.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_PARTIDR_Type {
    pub bits: BitVector<32>,
}

/// GICR_PENDBASER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47130-47137.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_PENDBASER_Type {
    pub bits: BitVector<64>,
}

/// GICR_PROPBASER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47141-47148.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_PROPBASER_Type {
    pub bits: BitVector<64>,
}

/// GICR_SETLPIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47317.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_SETLPIR_Type {
    pub bits: BitVector<64>,
}

/// GICR_STATUSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53331-53332.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_STATUSR_Type {
    pub bits: BitVector<32>,
}

/// GICR_SYNCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53257.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_SYNCR_Type {
    pub bits: BitVector<32>,
}

/// GICR_VPENDBASER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47212-47226.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_VPENDBASER_Type {
    pub bits: BitVector<64>,
}

/// GICR_VPROPBASER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47152-47165.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_VPROPBASER_Type {
    pub bits: BitVector<64>,
}

/// GICR_VSGIPENDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53144.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_VSGIPENDR_Type {
    pub bits: BitVector<32>,
}

/// GICR_VSGIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52827.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_VSGIR_Type {
    pub bits: BitVector<32>,
}

/// GICR_WAKER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52775-52776.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICR_WAKER_Type {
    pub bits: BitVector<32>,
}

/// GICV_ABPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52923.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_ABPR_Type {
    pub bits: BitVector<32>,
}

/// GICV_AEOIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52915.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_AEOIR_Type {
    pub bits: BitVector<32>,
}

/// GICV_AHPPIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52952.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_AHPPIR_Type {
    pub bits: BitVector<32>,
}

/// GICV_AIAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52944.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_AIAR_Type {
    pub bits: BitVector<32>,
}

/// GICV_BPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52975.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_BPR_Type {
    pub bits: BitVector<32>,
}

/// GICV_CTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53119-53127.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_CTLR_Type {
    pub bits: BitVector<32>,
}

/// GICV_DIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52796.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_DIR_Type {
    pub bits: BitVector<32>,
}

/// GICV_EOIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52983.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_EOIR_Type {
    pub bits: BitVector<32>,
}

/// GICV_HPPIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52784.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_HPPIR_Type {
    pub bits: BitVector<32>,
}

/// GICV_IAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52759.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_IAR_Type {
    pub bits: BitVector<32>,
}

/// GICV_PMR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53199.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_PMR_Type {
    pub bits: BitVector<32>,
}

/// GICV_RPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52948.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_RPR_Type {
    pub bits: BitVector<32>,
}

/// GICV_STATUSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52893-52894.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GICV_STATUSR_Type {
    pub bits: BitVector<32>,
}

/// GITS_CBASER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47305-47313.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_CBASER_Type {
    pub bits: BitVector<64>,
}

/// GITS_CREADR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47126.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_CREADR_Type {
    pub bits: BitVector<64>,
}

/// GITS_CTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52728-52735.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_CTLR_Type {
    pub bits: BitVector<32>,
}

/// GITS_CWRITER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47276.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_CWRITER_Type {
    pub bits: BitVector<64>,
}

/// GITS_IIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53290-53291.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_IIDR_Type {
    pub bits: BitVector<32>,
}

/// GITS_MPAMIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53216.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_MPAMIDR_Type {
    pub bits: BitVector<32>,
}

/// GITS_MPIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53273-53274.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_MPIDR_Type {
    pub bits: BitVector<32>,
}

/// GITS_PARTIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52780.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_PARTIDR_Type {
    pub bits: BitVector<32>,
}

/// GITS_SGIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47256.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_SGIR_Type {
    pub bits: BitVector<64>,
}

/// GITS_STATUSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53027-53036.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_STATUSR_Type {
    pub bits: BitVector<32>,
}

/// GITS_TYPER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47230-47252.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GITS_TYPER_Type {
    pub bits: BitVector<64>,
}

/// GMID_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46656.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GMID_EL1_Type {
    pub bits: BitVector<64>,
}

/// GPCCR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L19399-19409.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GPCCR_EL3_Type {
    pub bits: BitVector<64>,
}

/// GPTBR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L19520.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GPTBR_EL3_Type {
    pub bits: BitVector<64>,
}

/// HAFGRTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44066-44106.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HAFGRTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HCPTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7138-7146.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HCPTR_Type {
    pub bits: BitVector<32>,
}

/// HCR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14496-14507.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HCR2_Type {
    pub bits: BitVector<32>,
}

/// HCRX_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L10963-10986.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HCRX_EL2_Type {
    pub bits: BitVector<64>,
}

/// HCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L3274-3337.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8367-8398.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HCR_Type {
    pub bits: BitVector<32>,
}

/// HDCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L3914-3931.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HDCR_Type {
    pub bits: BitVector<32>,
}

/// HDFGRTR2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45688-45710.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HDFGRTR2_EL2_Type {
    pub bits: BitVector<64>,
}

/// HDFGRTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43898-43950.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HDFGRTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HDFGWTR2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45140-45160.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HDFGWTR2_EL2_Type {
    pub bits: BitVector<64>,
}

/// HDFGWTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46042-46088.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HDFGWTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HFGITR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L31764-31827.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HFGITR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HFGRTR2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45548-45549.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HFGRTR2_EL2_Type {
    pub bits: BitVector<64>,
}

/// HFGRTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45885-45950.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HFGRTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HFGWTR2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45976.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HFGWTR2_EL2_Type {
    pub bits: BitVector<64>,
}

/// HFGWTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44916-44968.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HFGWTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HMAIR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L37171-37172.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HMAIR0_Type {
    pub bits: BitVector<32>,
}

/// HMAIR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L37184-37185.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HMAIR1_Type {
    pub bits: BitVector<32>,
}

/// HPFAR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9266.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HPFAR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HPFAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L13824.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HPFAR_Type {
    pub bits: BitVector<32>,
}

/// HRMR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48301.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HRMR_Type {
    pub bits: BitVector<32>,
}

/// HSCTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8533-8548.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HSCTLR_Type {
    pub bits: BitVector<32>,
}

/// HSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L13845.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HSR_Type {
    pub bits: BitVector<32>,
}

/// HSTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34095-34111.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HSTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// HSTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L41535-41551.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HSTR_Type {
    pub bits: BitVector<32>,
}

/// HTCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L37197-37208.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HTCR_Type {
    pub bits: BitVector<32>,
}

/// HTRFCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39769-39770.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HTRFCR_Type {
    pub bits: BitVector<32>,
}

/// HTTBR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12962.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HTTBR_Type {
    pub bits: BitVector<64>,
}

/// ICC_AP1R_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44756.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_AP1R_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_ASGI1R_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45516-45525.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_ASGI1R_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_ASGI1R_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46456-46465.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_ASGI1R_Type {
    pub bits: BitVector<64>,
}

/// ICC_BPR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44110.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_BPR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_BPR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48843.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_BPR0_Type {
    pub bits: BitVector<32>,
}

/// ICC_BPR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45604.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_BPR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_BPR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48556.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_BPR1_Type {
    pub bits: BitVector<32>,
}

/// ICC_CTLR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44701-44712.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_CTLR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_CTLR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45445-45461.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_CTLR_EL3_Type {
    pub bits: BitVector<64>,
}

/// ICC_CTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49584-49595.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_CTLR_Type {
    pub bits: BitVector<32>,
}

/// ICC_DIR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44460.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_DIR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_DIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49295.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_DIR_Type {
    pub bits: BitVector<32>,
}

/// ICC_EOIR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46351.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_EOIR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_EOIR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48971.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_EOIR0_Type {
    pub bits: BitVector<32>,
}

/// ICC_EOIR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45642.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_EOIR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_EOIR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48775.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_EOIR1_Type {
    pub bits: BitVector<32>,
}

/// ICC_HPPIR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44053.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_HPPIR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_HPPIR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50373.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_HPPIR0_Type {
    pub bits: BitVector<32>,
}

/// ICC_HPPIR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43985.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_HPPIR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_HPPIR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51956.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_HPPIR1_Type {
    pub bits: BitVector<32>,
}

/// ICC_HSRE_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50031-50032.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_HSRE_Type {
    pub bits: BitVector<32>,
}

/// ICC_IAR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44614.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IAR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_IAR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47952.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IAR0_Type {
    pub bits: BitVector<32>,
}

/// ICC_IAR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46186.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IAR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_IAR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47589.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IAR1_Type {
    pub bits: BitVector<32>,
}

/// ICC_IGRPEN0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45596.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IGRPEN0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_IGRPEN0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49464.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IGRPEN0_Type {
    pub bits: BitVector<32>,
}

/// ICC_IGRPEN1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44504.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IGRPEN1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_IGRPEN1_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45294-45295.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IGRPEN1_EL3_Type {
    pub bits: BitVector<64>,
}

/// ICC_IGRPEN1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49143.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_IGRPEN1_Type {
    pub bits: BitVector<32>,
}

/// ICC_MCTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49275-49291.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_MCTLR_Type {
    pub bits: BitVector<32>,
}

/// ICC_MGRPEN1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50709.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_MGRPEN1_Type {
    pub bits: BitVector<32>,
}

/// ICC_MSRE_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49098-49099.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_MSRE_Type {
    pub bits: BitVector<32>,
}

/// ICC_NMIAR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44538.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_NMIAR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_PMR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L2783.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_PMR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_PMR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50169.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_PMR_Type {
    pub bits: BitVector<32>,
}

/// ICC_RPR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44245-44246.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_RPR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_RPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49056.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_RPR_Type {
    pub bits: BitVector<32>,
}

/// ICC_SGI0R_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44272-44281.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SGI0R_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_SGI0R_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46405-46414.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SGI0R_Type {
    pub bits: BitVector<64>,
}

/// ICC_SGI1R_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45980-45989.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SGI1R_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_SGI1R_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46371-46380.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SGI1R_Type {
    pub bits: BitVector<64>,
}

/// ICC_SRE_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44546.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SRE_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICC_SRE_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45485-45486.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SRE_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICC_SRE_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46019-46020.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SRE_EL3_Type {
    pub bits: BitVector<64>,
}

/// ICC_SRE_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51977.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICC_SRE_Type {
    pub bits: BitVector<32>,
}

/// ICH_AP0R_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44576-44610.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_AP0R_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_AP0R_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50062-50096.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_AP0R_Type {
    pub bits: BitVector<32>,
}

/// ICH_AP1R_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44421-44456.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_AP1R_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_AP1R_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49776-49810.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_AP1R_Type {
    pub bits: BitVector<32>,
}

/// ICH_EISR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44212-44230.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_EISR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_EISR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49884-49902.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_EISR_Type {
    pub bits: BitVector<32>,
}

/// ICH_ELRSR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44250-44268.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_ELRSR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_ELRSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47550-47568.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_ELRSR_Type {
    pub bits: BitVector<32>,
}

/// ICH_HCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45574-45592.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_HCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_HCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50242-50259.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_HCR_Type {
    pub bits: BitVector<32>,
}

/// ICH_LRC_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51889-51896.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_LRC_Type {
    pub bits: BitVector<32>,
}

/// ICH_LR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43958-43967.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_LR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_LR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50534.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_LR_Type {
    pub bits: BitVector<32>,
}

/// ICH_MISR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44807-44817.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_MISR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_MISR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50891-50901.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_MISR_Type {
    pub bits: BitVector<32>,
}

/// ICH_VMCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45415-45426.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_VMCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_VMCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48520-48531.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_VMCR_Type {
    pub bits: BitVector<32>,
}

/// ICH_VTR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45756-45767.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_VTR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ICH_VTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48403-48413.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICH_VTR_Type {
    pub bits: BitVector<32>,
}

/// ICV_AP1R_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46170.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_AP1R_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_BPR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44837.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_BPR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_BPR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49863.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_BPR0_Type {
    pub bits: BitVector<32>,
}

/// ICV_BPR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44015.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_BPR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_BPR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49035.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_BPR1_Type {
    pub bits: BitVector<32>,
}

/// ICV_CTLR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45005-45015.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_CTLR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_CTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48110-48120.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_CTLR_Type {
    pub bits: BitVector<32>,
}

/// ICV_DIR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46182.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_DIR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_DIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51839.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_DIR_Type {
    pub bits: BitVector<32>,
}

/// ICV_EOIR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45823.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_EOIR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_EOIR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48361.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_EOIR0_Type {
    pub bits: BitVector<32>,
}

/// ICV_EOIR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44041.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_EOIR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_EOIR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50190.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_EOIR1_Type {
    pub bits: BitVector<32>,
}

/// ICV_HPPIR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44833.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_HPPIR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_HPPIR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50610.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_HPPIR0_Type {
    pub bits: BitVector<32>,
}

/// ICV_HPPIR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46276.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_HPPIR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_HPPIR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48237.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_HPPIR1_Type {
    pub bits: BitVector<32>,
}

/// ICV_IAR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44500.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IAR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_IAR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50993.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IAR0_Type {
    pub bits: BitVector<32>,
}

/// ICV_IAR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45023.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IAR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_IAR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48701.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IAR1_Type {
    pub bits: BitVector<32>,
}

/// ICV_IGRPEN0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45851.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IGRPEN0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_IGRPEN0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47982.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IGRPEN0_Type {
    pub bits: BitVector<32>,
}

/// ICV_IGRPEN1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45968.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IGRPEN1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_IGRPEN1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47771.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_IGRPEN1_Type {
    pub bits: BitVector<32>,
}

/// ICV_NMIAR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43954.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_NMIAR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_PMR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44829.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_PMR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_PMR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50589.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_PMR_Type {
    pub bits: BitVector<32>,
}

/// ICV_RPR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44825.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_RPR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ICV_RPR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48620.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ICV_RPR_Type {
    pub bits: BitVector<32>,
}

/// ID_AA64DFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43993-44011.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64DFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64DFR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44114-44127.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64DFR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64ISAR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44468-44485.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64ISAR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64ISAR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44182-44200.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64ISAR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64ISAR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45529-45544.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64ISAR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64MMFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45646-45662.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64MMFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64MMFR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45317-45335.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64MMFR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64MMFR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43877-43894.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64MMFR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64MMFR3_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45735-45752.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64MMFR3_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64MMFR4_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44204.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64MMFR4_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64PFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44399-44417.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64PFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64PFR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44378-44395.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64PFR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64PFR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45503-45504.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64PFR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64SMFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46582-46596.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64SMFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_AA64ZFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46355-46367.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_AA64ZFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_DFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45954-45964.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_DFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_DFR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47657-47667.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_DFR0_Type {
    pub bits: BitVector<32>,
}

/// ID_DFR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45827.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_DFR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_DFR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50513.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_DFR1_Type {
    pub bits: BitVector<32>,
}

/// ID_ISAR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45164-45173.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_ISAR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48662-48671.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR0_Type {
    pub bits: BitVector<32>,
}

/// ID_ISAR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44636-44646.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_ISAR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L52041-52051.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR1_Type {
    pub bits: BitVector<32>,
}

/// ID_ISAR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44019-44029.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_ISAR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51695-51705.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR2_Type {
    pub bits: BitVector<32>,
}

/// ID_ISAR3_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44846-44856.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR3_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_ISAR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50669-50679.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR3_Type {
    pub bits: BitVector<32>,
}

/// ID_ISAR4_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44622-44632.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR4_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_ISAR4_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50415-50425.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR4_Type {
    pub bits: BitVector<32>,
}

/// ID_ISAR5_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45557-45566.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR5_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_ISAR5_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50118-50127.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR5_Type {
    pub bits: BitVector<32>,
}

/// ID_ISAR6_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44742-44752.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR6_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_ISAR6_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48151-48161.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_ISAR6_Type {
    pub bits: BitVector<32>,
}

/// ID_MMFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45397-45407.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_MMFR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49832-49842.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR0_Type {
    pub bits: BitVector<32>,
}

/// ID_MMFR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45299-45309.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_MMFR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49004-49014.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR1_Type {
    pub bits: BitVector<32>,
}

/// ID_MMFR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43849-43859.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_MMFR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51014-51024.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR2_Type {
    pub bits: BitVector<32>,
}

/// ID_MMFR3_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46028-46038.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR3_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_MMFR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48003-48013.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR3_Type {
    pub bits: BitVector<32>,
}

/// ID_MMFR4_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43863-43873.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR4_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_MMFR4_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48744-48754.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR4_Type {
    pub bits: BitVector<32>,
}

/// ID_MMFR5_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44534.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR5_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_MMFR5_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48258.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_MMFR5_Type {
    pub bits: BitVector<32>,
}

/// ID_PFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45112-45122.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_PFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_PFR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51173-51183.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_PFR0_Type {
    pub bits: BitVector<32>,
}

/// ID_PFR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45771-45781.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_PFR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_PFR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50211-50221.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_PFR1_Type {
    pub bits: BitVector<32>,
}

/// ID_PFR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46292-46293.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_PFR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// ID_PFR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48331.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ID_PFR2_Type {
    pub bits: BitVector<32>,
}

/// IFSR32_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14337-14338.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct IFSR32_EL2_Type {
    pub bits: BitVector<64>,
}

/// IFSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14342-14343.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct IFSR_Type {
    pub bits: BitVector<32>,
}

/// ISR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44841-44842.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ISR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ISR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51066.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ISR_Type {
    pub bits: BitVector<32>,
}

/// LORC_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45499.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct LORC_EL1_Type {
    pub bits: BitVector<64>,
}

/// LOREA_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46174.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct LOREA_EL1_Type {
    pub bits: BitVector<64>,
}

/// LORID_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44170.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct LORID_EL1_Type {
    pub bits: BitVector<64>,
}

/// LORN_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45847.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct LORN_EL1_Type {
    pub bits: BitVector<64>,
}

/// LORSA_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45313.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct LORSA_EL1_Type {
    pub bits: BitVector<64>,
}

/// MAIR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L36902-36903.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR0_Type {
    pub bits: BitVector<32>,
}

/// MAIR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L36993-36994.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR1_Type {
    pub bits: BitVector<32>,
}

/// MAIR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20376-20386.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// MAIR2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20609-20619.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR2_EL2_Type {
    pub bits: BitVector<64>,
}

/// MAIR2_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20886-20896.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR2_EL3_Type {
    pub bits: BitVector<64>,
}

/// MAIRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1266-1276.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIRType {
    pub bits: BitVector<64>,
}

/// MAIR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20390-20400.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR_EL1_Type {
    pub bits: BitVector<64>,
}

/// MAIR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20623-20633.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR_EL2_Type {
    pub bits: BitVector<64>,
}

/// MAIR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20900-20910.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MAIR_EL3_Type {
    pub bits: BitVector<64>,
}

/// MDCCINT_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39459.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MDCCINT_EL1_Type {
    pub bits: BitVector<64>,
}

/// MDCCSR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4216.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MDCCSR_EL0_Type {
    pub bits: BitVector<64>,
}

/// MDCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L3933-3957.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MDCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// MDCR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4667-4700.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MDCR_EL3_Type {
    pub bits: BitVector<64>,
}

/// MDRAR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45286.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MDRAR_EL1_Type {
    pub bits: BitVector<64>,
}

/// MDSCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8300-8319.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MDSCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// MDSELR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34335.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MDSELR_EL1_Type {
    pub bits: BitVector<64>,
}

/// MECIDR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44868.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MECIDR_EL2_Type {
    pub bits: BitVector<64>,
}

/// MECID_A0_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24770.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MECID_A0_EL2_Type {
    pub bits: BitVector<64>,
}

/// MECID_A1_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24774.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MECID_A1_EL2_Type {
    pub bits: BitVector<64>,
}

/// MECID_P0_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24580.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MECID_P0_EL2_Type {
    pub bits: BitVector<64>,
}

/// MECID_P1_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24778.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MECID_P1_EL2_Type {
    pub bits: BitVector<64>,
}

/// MECID_RL_A_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24782.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MECID_RL_A_EL3_Type {
    pub bits: BitVector<64>,
}

/// MFAR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9276-9282.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MFAR_EL3_Type {
    pub bits: BitVector<64>,
}

/// MIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44234-44241.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// MIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51284-51291.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MIDR_Type {
    pub bits: BitVector<32>,
}

/// MPAM0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11633-11634.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAM0_EL1_Type {
    pub bits: BitVector<64>,
}

/// MPAM1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11189-11198.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAM1_EL1_Type {
    pub bits: BitVector<64>,
}

/// MPAM2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11145-11159.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAM2_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAM3_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11163-11177.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAM3_EL3_Type {
    pub bits: BitVector<64>,
}

/// MPAMHCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11427-11433.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMHCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11389-11400.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// MPAMSM_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11638.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMSM_EL1_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM0_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11437-11443.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM0_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM1_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11485-11491.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM1_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11495-11501.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM2_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM3_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11505-11511.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM3_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM4_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11515-11521.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM4_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM5_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11525-11531.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM5_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM6_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11535-11541.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM6_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPM7_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11545-11551.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPM7_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPAMVPMV_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L11447-11481.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMVPMV_EL2_Type {
    pub bits: BitVector<64>,
}

/// MPIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46190-46198.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// MPIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51860-51868.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPIDR_Type {
    pub bits: BitVector<32>,
}

/// MVBAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14124.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MVBAR_Type {
    pub bits: BitVector<32>,
}

/// MVFR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43971-43981.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MVFR0_EL1_Type {
    pub bits: BitVector<64>,
}

/// MVFR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49979-49989.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MVFR0_Type {
    pub bits: BitVector<32>,
}

/// MVFR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45126-45136.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MVFR1_EL1_Type {
    pub bits: BitVector<64>,
}

/// MVFR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49553-49563.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MVFR1_Type {
    pub bits: BitVector<32>,
}

/// MVFR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45997.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MVFR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// MVFR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49077.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MVFR2_Type {
    pub bits: BitVector<32>,
}

/// NMRR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L36996-37014.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct NMRR_Type {
    pub bits: BitVector<32>,
}

/// NSACR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7158-7159.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct NSACR_Type {
    pub bits: BitVector<32>,
}

/// OSDLR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L7914.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct OSDLR_EL1_Type {
    pub bits: BitVector<64>,
}

/// OSECCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L10177.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct OSECCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// OSLAR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44049.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct OSLAR_EL1_Type {
    pub bits: BitVector<64>,
}

/// OSLSR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8323-8324.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct OSLSR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PAR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39138-39153.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PAR_EL1_Type {
    pub bits: BitVector<128>,
}

/// PAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39104-39120.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PAR_Type {
    pub bits: BitVector<64>,
}

/// PFAR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9286-9287.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PFAR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PFAR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9291-9292.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PFAR_EL2_Type {
    pub bits: BitVector<64>,
}

/// PIRE0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20404-20422.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PIRE0_EL1_Type {
    pub bits: BitVector<64>,
}

/// PIRE0_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20747-20765.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PIRE0_EL2_Type {
    pub bits: BitVector<64>,
}

/// PIR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20426-20444.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PIR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PIR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20637-20655.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PIR_EL2_Type {
    pub bits: BitVector<64>,
}

/// PIR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20914-20932.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PIR_EL3_Type {
    pub bits: BitVector<64>,
}

/// PMAUTHSTATUS_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53446-53456.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMAUTHSTATUS_Type {
    pub bits: BitVector<32>,
}

/// PMBIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L35095-35096.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMBIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMBLIMITR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6554-6555.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMBLIMITR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMBPTR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34772.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMBPTR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMBSR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6559-6570.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMBSR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMCCFILTR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4704-4717.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCCFILTR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMCCFILTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4721-4729.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCCFILTR_Type {
    pub bits: BitVector<32>,
}

/// PMCCNTR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34423.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCCNTR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMCCNTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43539.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCCNTR_Type {
    pub bits: BitVector<64>,
}

/// PMCCNTSVR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44864.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCCNTSVR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMCEID0_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46206-46272.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCEID0_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMCEID0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L48055-48089.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCEID0_Type {
    pub bits: BitVector<32>,
}

/// PMCEID1_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46092-46158.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCEID1_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMCEID1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47875-47909.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCEID1_Type {
    pub bits: BitVector<32>,
}

/// PMCEID2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50446-50480.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCEID2_Type {
    pub bits: BitVector<32>,
}

/// PMCEID3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50318-50352.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCEID3_Type {
    pub bits: BitVector<32>,
}

/// PMCFGR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47321-47334.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCFGR_Type {
    pub bits: BitVector<64>,
}

/// PMCGCR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53460.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCGCR0_Type {
    pub bits: BitVector<32>,
}

/// PMCIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53464.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCIDR0_Type {
    pub bits: BitVector<32>,
}

/// PMCIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53468.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCIDR1_Type {
    pub bits: BitVector<32>,
}

/// PMCIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53472.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCIDR2_Type {
    pub bits: BitVector<32>,
}

/// PMCIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53476.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCIDR3_Type {
    pub bits: BitVector<32>,
}

/// PMCNTENCLR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4260-4295.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCNTENCLR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMCNTENCLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4299-4333.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCNTENCLR_Type {
    pub bits: BitVector<32>,
}

/// PMCNTENSET_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4741-4776.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCNTENSET_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMCNTENSET_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4780-4814.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCNTENSET_Type {
    pub bits: BitVector<32>,
}

/// PMCNTEN_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47338-47373.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCNTEN_Type {
    pub bits: BitVector<64>,
}

/// PMCR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4345-4360.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4364-4378.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMCR_Type {
    pub bits: BitVector<32>,
}

/// PMDEVID_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53480.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMDEVID_Type {
    pub bits: BitVector<32>,
}

/// PMDEVTYPE_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53484.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMDEVTYPE_Type {
    pub bits: BitVector<32>,
}

/// PMECR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44033.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMECR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMEVCNTSVR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44464.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMEVCNTSVR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMEVTYPER_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4826-4845.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMEVTYPER_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMEVTYPER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4849-4859.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMEVTYPER_Type {
    pub bits: BitVector<32>,
}

/// PMIAR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45389.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMIAR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMICFILTR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4871-4886.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMICFILTR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMICNTR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5135.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMICNTR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMICNTSVR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45282.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMICNTSVR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMIIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47394-47395.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMIIDR_Type {
    pub bits: BitVector<64>,
}

/// PMINTENCLR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4390-4425.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMINTENCLR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMINTENCLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4429-4463.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMINTENCLR_Type {
    pub bits: BitVector<32>,
}

/// PMINTENSET_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44872-44907.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMINTENSET_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMINTENSET_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50738-50772.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMINTENSET_Type {
    pub bits: BitVector<32>,
}

/// PMINTEN_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47399-47434.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMINTEN_Type {
    pub bits: BitVector<64>,
}

/// PMITCTRL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53488.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMITCTRL_Type {
    pub bits: BitVector<32>,
}

/// PMLAR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53492.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMLAR_Type {
    pub bits: BitVector<32>,
}

/// PMLSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39392.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMLSR_Type {
    pub bits: BitVector<32>,
}

/// PMMIR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44489-44496.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMMIR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMMIR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49103-49110.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMMIR_Type {
    pub bits: BitVector<32>,
}

/// PMOVSCLR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4475-4510.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMOVSCLR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMOVSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4514-4548.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMOVSR_Type {
    pub bits: BitVector<32>,
}

/// PMOVSSET_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5139-5174.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMOVSSET_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMOVSSET_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L43421-43455.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMOVSSET_Type {
    pub bits: BitVector<32>,
}

/// PMOVS_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47455-47490.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMOVS_Type {
    pub bits: BitVector<64>,
}

/// PMPCSCTL_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47511.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMPCSCTL_Type {
    pub bits: BitVector<64>,
}

/// PMPCSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39754-39761.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMPCSR_Type {
    pub bits: BitVector<64>,
}

/// PMPIDR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53496.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMPIDR0_Type {
    pub bits: BitVector<32>,
}

/// PMPIDR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53500.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMPIDR1_Type {
    pub bits: BitVector<32>,
}

/// PMPIDR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53504.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMPIDR2_Type {
    pub bits: BitVector<32>,
}

/// PMPIDR3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53508.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMPIDR3_Type {
    pub bits: BitVector<32>,
}

/// PMPIDR4_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L53512.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMPIDR4_Type {
    pub bits: BitVector<32>,
}

/// PMSCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6544-6545.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6549-6550.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// PMSDSFR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34785-34851.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSDSFR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSELR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44697.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSELR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMSELR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49316.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSELR_Type {
    pub bits: BitVector<32>,
}

/// PMSEVFR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34855-34904.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSEVFR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSFCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34908-34918.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSFCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSICR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34672.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSICR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6520-6536.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSIRR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34676.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSIRR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSLATFR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34922.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSLATFR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSNEVFR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L34926-34975.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSNEVFR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSSCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44155.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSSCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMSWINC_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45339-45372.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSWINC_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMSWINC_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51504-51537.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMSWINC_Type {
    pub bits: BitVector<32>,
}

/// PMUACR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44650-44685.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMUACR_EL1_Type {
    pub bits: BitVector<64>,
}

/// PMUSERENR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44976-44985.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMUSERENR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMUSERENR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47822-47823.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMUSERENR_Type {
    pub bits: BitVector<32>,
}

/// PMVCIDSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L47515-47516.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMVCIDSR_Type {
    pub bits: BitVector<64>,
}

/// PMVIDSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39765.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMVIDSR_Type {
    pub bits: BitVector<32>,
}

/// PMXEVCNTR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45972.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMXEVCNTR_EL0_Type {
    pub bits: BitVector<64>,
}

/// PMZR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45243-45278.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PMZR_EL0_Type {
    pub bits: BitVector<64>,
}

/// POR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24298-24316.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct POR_EL0_Type {
    pub bits: BitVector<64>,
}

/// POR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24210-24228.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct POR_EL1_Type {
    pub bits: BitVector<64>,
}

/// POR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24232-24250.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct POR_EL2_Type {
    pub bits: BitVector<64>,
}

/// POR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24254-24272.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct POR_EL3_Type {
    pub bits: BitVector<64>,
}

/// PRRR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L36905-36927.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PRRR_Type {
    pub bits: BitVector<32>,
}

/// RCWMASK_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L28497.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RCWMASK_EL1_Type {
    pub bits: BitVector<128>,
}

/// RCWSMASK_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L28501.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RCWSMASK_EL1_Type {
    pub bits: BitVector<128>,
}

/// RGSR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L22682.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RGSR_EL1_Type {
    pub bits: BitVector<64>,
}

/// RMR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44045.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RMR_EL1_Type {
    pub bits: BitVector<64>,
}

/// RMR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44693.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RMR_EL2_Type {
    pub bits: BitVector<64>,
}

/// RMR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45570.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RMR_EL3_Type {
    pub bits: BitVector<64>,
}

/// RMR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50280.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RMR_Type {
    pub bits: BitVector<32>,
}

/// RNDRRS_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45843.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RNDRRS_Type {
    pub bits: BitVector<64>,
}

/// RNDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45881.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RNDR_Type {
    pub bits: BitVector<64>,
}

/// RVBAR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33484.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RVBAR_EL1_Type {
    pub bits: BitVector<64>,
}

/// RVBAR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33488.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RVBAR_EL2_Type {
    pub bits: BitVector<64>,
}

/// RVBAR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33492.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RVBAR_EL3_Type {
    pub bits: BitVector<64>,
}

/// S1PIRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1278-1296.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S1PIRType {
    pub bits: BitVector<64>,
}

/// S1PORType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1298-1316.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S1PORType {
    pub bits: BitVector<64>,
}

/// S2PIRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1318-1336.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S2PIRType {
    pub bits: BitVector<64>,
}

/// S2PIR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L25761-25779.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S2PIR_EL2_Type {
    pub bits: BitVector<64>,
}

/// S2POR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L26103-26121.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S2POR_EL1_Type {
    pub bits: BitVector<64>,
}

/// SCRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1212-1229.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCRType {
    pub bits: BitVector<64>,
}

/// SCR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L3363-3412.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCR_EL3_Type {
    pub bits: BitVector<64>,
}

/// SCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L3457-3472.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCR_Type {
    pub bits: BitVector<32>,
}

/// SCTLR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14885-14886.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// SCTLR2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L14890-14898.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLR2_EL2_Type {
    pub bits: BitVector<64>,
}

/// SCTLR2_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20936-20937.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLR2_EL3_Type {
    pub bits: BitVector<64>,
}

/// SCTLRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1231-1264.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLRType {
    pub bits: BitVector<64>,
}

/// SCTLR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8622-8681.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLR_EL1_Type {
    pub bits: BitVector<64>,
}

/// SCTLR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8550-8608.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLR_EL2_Type {
    pub bits: BitVector<64>,
}

/// SCTLR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8685-8712.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLR_EL3_Type {
    pub bits: BitVector<64>,
}

/// SCTLR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L8716-8740.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SCTLR_Type {
    pub bits: BitVector<32>,
}

/// SDCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4890-4901.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SDCR_Type {
    pub bits: BitVector<32>,
}

/// SDER32_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4905.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SDER32_EL2_Type {
    pub bits: BitVector<64>,
}

/// SDER32_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4909.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SDER32_EL3_Type {
    pub bits: BitVector<64>,
}

/// SDER_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L4921.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SDER_Type {
    pub bits: BitVector<32>,
}

/// SMCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6955.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SMCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// SMCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6959.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SMCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// SMCR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6963.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SMCR_EL3_Type {
    pub bits: BitVector<64>,
}

/// SMIDR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44911-44912.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SMIDR_EL1_Type {
    pub bits: BitVector<64>,
}

/// SMPRIMAP_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45859-45877.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SMPRIMAP_EL2_Type {
    pub bits: BitVector<64>,
}

/// SMPRI_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44972.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SMPRI_EL1_Type {
    pub bits: BitVector<64>,
}

/// SPMACCESSR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44329-44363.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPMACCESSR_EL1_Type {
    pub bits: BitVector<64>,
}

/// SPMACCESSR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45198-45232.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPMACCESSR_EL2_Type {
    pub bits: BitVector<64>,
}

/// SPMACCESSR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L46297-46331.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPMACCESSR_EL3_Type {
    pub bits: BitVector<64>,
}

/// SPMSELR_EL0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L44367.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPMSELR_EL0_Type {
    pub bits: BitVector<64>,
}

/// SPSR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9515-9541.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_EL1_Type {
    pub bits: BitVector<64>,
}

/// SPSR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9545-9571.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_EL2_Type {
    pub bits: BitVector<64>,
}

/// SPSR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9575-9601.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_EL3_Type {
    pub bits: BitVector<64>,
}

/// SPSR_abt_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9605-9625.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_abt_Type {
    pub bits: BitVector<64>,
}

/// SPSR_fiq_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9629-9649.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_fiq_Type {
    pub bits: BitVector<64>,
}

/// SPSR_hyp_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9653-9673.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_hyp_Type {
    pub bits: BitVector<32>,
}

/// SPSR_irq_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9694-9714.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_irq_Type {
    pub bits: BitVector<64>,
}

/// SPSR_mon_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9718-9738.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_mon_Type {
    pub bits: BitVector<32>,
}

/// SPSR_svc_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9742-9762.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_svc_Type {
    pub bits: BitVector<32>,
}

/// SPSR_und_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L9783-9803.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SPSR_und_Type {
    pub bits: BitVector<64>,
}

/// TCR2_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20448-20460.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TCR2_EL1_Type {
    pub bits: BitVector<64>,
}

/// TCR2_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20659-20675.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TCR2_EL2_Type {
    pub bits: BitVector<64>,
}

/// TCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5943-5985.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// TCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L5989-6038.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// TCR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6042-6070.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TCR_EL3_Type {
    pub bits: BitVector<64>,
}

/// TFSRE0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L22962.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TFSRE0_EL1_Type {
    pub bits: BitVector<64>,
}

/// TFSR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L22966.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TFSR_EL1_Type {
    pub bits: BitVector<64>,
}

/// TFSR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L22970.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TFSR_EL2_Type {
    pub bits: BitVector<64>,
}

/// TFSR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L22974.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TFSR_EL3_Type {
    pub bits: BitVector<64>,
}

/// TLBTR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L50713.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLBTR_Type {
    pub bits: BitVector<32>,
}

/// TRFCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39787.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TRFCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// TRFCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39772-39773.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TRFCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// TRFCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L39791.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TRFCR_Type {
    pub bits: BitVector<32>,
}

/// TTBCR2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L37080-37092.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBCR2_Type {
    pub bits: BitVector<32>,
}

/// TTBCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12845-12863.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBCR_Type {
    pub bits: BitVector<32>,
}

/// TTBR0_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12950.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBR0_EL1_Type {
    pub bits: BitVector<128>,
}

/// TTBR0_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12966.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBR0_EL2_Type {
    pub bits: BitVector<128>,
}

/// TTBR0_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L20941.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBR0_EL3_Type {
    pub bits: BitVector<64>,
}

/// TTBR0_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12921-12932.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBR0_Type {
    pub bits: BitVector<64>,
}

/// TTBR1_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L13007.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBR1_EL1_Type {
    pub bits: BitVector<128>,
}

/// TTBR1_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L13019.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBR1_EL2_Type {
    pub bits: BitVector<128>,
}

/// TTBR1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L12978-12989.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTBR1_Type {
    pub bits: BitVector<64>,
}

/// VDFSR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33307.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VDFSR_Type {
    pub bits: BitVector<32>,
}

/// VDISR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33324-33334.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VDISR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VDISR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33338-33346.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VDISR_Type {
    pub bits: BitVector<32>,
}

/// VMECID_A_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L26568.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VMECID_A_EL2_Type {
    pub bits: BitVector<64>,
}

/// VMECID_P_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L24584.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VMECID_P_EL2_Type {
    pub bits: BitVector<64>,
}

/// VMPIDR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45723-45731.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VMPIDR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VMPIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L49114-49122.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VMPIDR_Type {
    pub bits: BitVector<32>,
}

/// VNCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L28747.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VNCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VPIDR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L45430-45437.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VPIDR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VPIDR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L51476-51483.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VPIDR_Type {
    pub bits: BitVector<32>,
}

/// VSESR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L33309-33310.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VSESR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VSTCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L25853-25861.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VSTCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VSTTBR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L25865.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VSTTBR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VTCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23202-23230.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VTCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// VTCR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L37692-37704.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VTCR_Type {
    pub bits: BitVector<32>,
}

/// VTTBR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23238.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VTTBR_EL2_Type {
    pub bits: BitVector<128>,
}

/// VTTBR_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L23234.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct VTTBR_Type {
    pub bits: BitVector<64>,
}

/// ZCR_EL1_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6889.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ZCR_EL1_Type {
    pub bits: BitVector<64>,
}

/// ZCR_EL2_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6893.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ZCR_EL2_Type {
    pub bits: BitVector<64>,
}

/// ZCR_EL3_Type
///
/// Generated from the Sail sources at `src/v8_base.sail` L6897.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ZCR_EL3_Type {
    pub bits: BitVector<64>,
}

/// Feature
///
/// Generated from the Sail sources at `src/v8_base.sail` L72-331.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Feature {
    FEAT_AA32EL0,
    FEAT_AA32EL1,
    FEAT_AA32EL2,
    FEAT_AA32EL3,
    FEAT_AA64EL0,
    FEAT_AA64EL1,
    FEAT_AA64EL2,
    FEAT_AA64EL3,
    FEAT_EL0,
    FEAT_EL1,
    FEAT_EL2,
    FEAT_EL3,
    FEAT_AES,
    FEAT_AdvSIMD,
    FEAT_CSV2_1p1,
    FEAT_CSV2_1p2,
    FEAT_CSV2_2,
    FEAT_CSV2_3,
    FEAT_DoubleLock,
    FEAT_ETMv4,
    FEAT_ETMv4p1,
    FEAT_ETMv4p2,
    FEAT_ETMv4p3,
    FEAT_ETMv4p4,
    FEAT_ETMv4p5,
    FEAT_ETMv4p6,
    FEAT_ETS2,
    FEAT_FP,
    FEAT_GICv3,
    FEAT_GICv3_LEGACY,
    FEAT_GICv3_TDIR,
    FEAT_GICv3p1,
    FEAT_GICv4,
    FEAT_GICv4p1,
    FEAT_IVIPT,
    FEAT_PCSRv8,
    FEAT_PMULL,
    FEAT_PMUv3,
    FEAT_PMUv3_EXT,
    FEAT_PMUv3_EXT32,
    FEAT_SHA1,
    FEAT_SHA256,
    FEAT_TRC_EXT,
    FEAT_TRC_SR,
    FEAT_nTLBPA,
    FEAT_CRC32,
    FEAT_Debugv8p1,
    FEAT_HAFDBS,
    FEAT_HPDS,
    FEAT_LOR,
    FEAT_LSE,
    FEAT_PAN,
    FEAT_PMUv3p1,
    FEAT_RDM,
    FEAT_VHE,
    FEAT_VMID16,
    FEAT_AA32BF16,
    FEAT_AA32HPD,
    FEAT_AA32I8MM,
    FEAT_ASMv8p2,
    FEAT_DPB,
    FEAT_Debugv8p2,
    FEAT_EDHSR,
    FEAT_F32MM,
    FEAT_F64MM,
    FEAT_FP16,
    FEAT_HPDS2,
    FEAT_I8MM,
    FEAT_IESB,
    FEAT_LPA,
    FEAT_LSMAOC,
    FEAT_LVA,
    FEAT_MPAM,
    FEAT_PAN2,
    FEAT_PCSRv8p2,
    FEAT_RAS,
    FEAT_SHA3,
    FEAT_SHA512,
    FEAT_SM3,
    FEAT_SM4,
    FEAT_SPE,
    FEAT_SVE,
    FEAT_TTCNP,
    FEAT_UAO,
    FEAT_VPIPT,
    FEAT_XNX,
    FEAT_CCIDX,
    FEAT_CONSTPACFIELD,
    FEAT_EPAC,
    FEAT_FCMA,
    FEAT_FPAC,
    FEAT_FPACCOMBINE,
    FEAT_JSCVT,
    FEAT_LRCPC,
    FEAT_NV,
    FEAT_PACIMP,
    FEAT_PACQARMA3,
    FEAT_PACQARMA5,
    FEAT_PAuth,
    FEAT_SPEv1p1,
    FEAT_AMUv1,
    FEAT_BBM,
    FEAT_CNTSC,
    FEAT_DIT,
    FEAT_Debugv8p4,
    FEAT_DotProd,
    FEAT_DoubleFault,
    FEAT_FHM,
    FEAT_FlagM,
    FEAT_IDST,
    FEAT_LRCPC2,
    FEAT_LSE2,
    FEAT_NV2,
    FEAT_PMUv3p4,
    FEAT_RASSAv1p1,
    FEAT_RASv1p1,
    FEAT_S2FWB,
    FEAT_SEL2,
    FEAT_TLBIOS,
    FEAT_TLBIRANGE,
    FEAT_TRF,
    FEAT_TTL,
    FEAT_TTST,
    FEAT_BTI,
    FEAT_CSV2,
    FEAT_CSV3,
    FEAT_DPB2,
    FEAT_E0PD,
    FEAT_EVT,
    FEAT_ExS,
    FEAT_FRINTTS,
    FEAT_FlagM2,
    FEAT_GTG,
    FEAT_MTE,
    FEAT_MTE2,
    FEAT_PMUv3p5,
    FEAT_RNG,
    FEAT_RNG_TRAP,
    FEAT_SB,
    FEAT_SPECRES,
    FEAT_SSBS,
    FEAT_SSBS2,
    FEAT_AMUv1p1,
    FEAT_BF16,
    FEAT_DGH,
    FEAT_ECV,
    FEAT_FGT,
    FEAT_HPMN0,
    FEAT_MPAMv0p1,
    FEAT_MPAMv1p1,
    FEAT_MTPMU,
    FEAT_PAuth2,
    FEAT_TWED,
    FEAT_AFP,
    FEAT_EBF16,
    FEAT_HCX,
    FEAT_LPA2,
    FEAT_LS64,
    FEAT_LS64_ACCDATA,
    FEAT_LS64_V,
    FEAT_MTE3,
    FEAT_PAN3,
    FEAT_PMUv3p7,
    FEAT_RPRES,
    FEAT_SPEv1p2,
    FEAT_WFxT,
    FEAT_XS,
    FEAT_CMOW,
    FEAT_Debugv8p8,
    FEAT_GICv3_NMI,
    FEAT_HBC,
    FEAT_MOPS,
    FEAT_NMI,
    FEAT_PMUv3_EXT64,
    FEAT_PMUv3_TH,
    FEAT_PMUv3p8,
    FEAT_SCTLR2,
    FEAT_SPEv1p3,
    FEAT_TCR2,
    FEAT_TIDCP1,
    FEAT_ADERR,
    FEAT_AIE,
    FEAT_ANERR,
    FEAT_CLRBHB,
    FEAT_CSSC,
    FEAT_Debugv8p9,
    FEAT_DoubleFault2,
    FEAT_ECBHB,
    FEAT_FGT2,
    FEAT_HAFT,
    FEAT_LRCPC3,
    FEAT_MTE4,
    FEAT_MTE_ASYM_FAULT,
    FEAT_MTE_ASYNC,
    FEAT_MTE_CANONICAL_TAGS,
    FEAT_MTE_NO_ADDRESS_TAGS,
    FEAT_MTE_PERM,
    FEAT_MTE_STORE_ONLY,
    FEAT_MTE_TAGGED_FAR,
    FEAT_PCSRv8p9,
    FEAT_PFAR,
    FEAT_PMUv3_EDGE,
    FEAT_PMUv3_ICNTR,
    FEAT_PMUv3_SS,
    FEAT_PMUv3p9,
    FEAT_PRFMSLC,
    FEAT_RASSAv2,
    FEAT_RASv2,
    FEAT_RPRFM,
    FEAT_S1PIE,
    FEAT_S1POE,
    FEAT_S2PIE,
    FEAT_S2POE,
    FEAT_SPECRES2,
    FEAT_SPE_CRR,
    FEAT_SPE_FDS,
    FEAT_SPEv1p4,
    FEAT_SPMU,
    FEAT_THE,
    FEAT_DoPD,
    FEAT_ETE,
    FEAT_SVE2,
    FEAT_SVE_AES,
    FEAT_SVE_BitPerm,
    FEAT_SVE_PMULL128,
    FEAT_SVE_SHA3,
    FEAT_SVE_SM4,
    FEAT_TME,
    FEAT_TRBE,
    FEAT_ETEv1p1,
    FEAT_BRBE,
    FEAT_ETEv1p2,
    FEAT_RME,
    FEAT_SME,
    FEAT_SME_F64F64,
    FEAT_SME_FA64,
    FEAT_SME_I16I64,
    FEAT_BRBEv1p1,
    FEAT_MEC,
    FEAT_SME2,
    FEAT_ABLE,
    FEAT_CHK,
    FEAT_D128,
    FEAT_EBEP,
    FEAT_ETEv1p3,
    FEAT_GCS,
    FEAT_ITE,
    FEAT_LSE128,
    FEAT_LVA3,
    FEAT_SEBEP,
    FEAT_SME2p1,
    FEAT_SME_F16F16,
    FEAT_SVE2p1,
    FEAT_SVE_B16B16,
    FEAT_SYSINSTR128,
    FEAT_SYSREG128,
    FEAT_TRBE_EXT,
    FEAT_TRBE_MPAM,
}

/// ArchVersion
///
/// Generated from the Sail sources at `src/v8_base.sail` L333-349.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ArchVersion {
    v8Ap0,
    v8Ap1,
    v8Ap2,
    v8Ap3,
    v8Ap4,
    v8Ap5,
    v8Ap6,
    v8Ap7,
    v8Ap8,
    v8Ap9,
    v9Ap0,
    v9Ap1,
    v9Ap2,
    v9Ap3,
    v9Ap4,
}

/// Signal
///
/// Generated from the Sail sources at `src/v8_base.sail` L1380.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Signal {
    Signal_Low,
    Signal_High,
}

pub const CYCLE_COUNTER_ID: i128 = 31;

pub const INSTRUCTION_COUNTER_ID: i128 = 32;

pub const PMU_EVENT_SW_INCR: BitVector<16> = BitVector::<16>::new(0b0000000000000000);

pub const PMU_EVENT_L1D_CACHE_REFILL: BitVector<16> = BitVector::<16>::new(0b0000000000000011);

pub const PMU_EVENT_L1D_CACHE: BitVector<16> = BitVector::<16>::new(0b0000000000000100);

pub const PMU_EVENT_INST_RETIRED: BitVector<16> = BitVector::<16>::new(0b0000000000001000);

pub const PMU_EVENT_EXC_TAKEN: BitVector<16> = BitVector::<16>::new(0b0000000000001001);

pub const PMU_EVENT_BR_MIS_PRED: BitVector<16> = BitVector::<16>::new(0b0000000000010000);

pub const PMU_EVENT_CPU_CYCLES: BitVector<16> = BitVector::<16>::new(0b0000000000010001);

pub const PMU_EVENT_INST_SPEC: BitVector<16> = BitVector::<16>::new(0b0000000000011011);

pub const PMU_EVENT_CHAIN: BitVector<16> = BitVector::<16>::new(0b0000000000011110);

pub const PMU_EVENT_BR_MIS_PRED_RETIRED: BitVector<16> = BitVector::<16>::new(0b0000000000100010);

pub const PMU_EVENT_L1D_TLB: BitVector<16> = BitVector::<16>::new(0b0000000000100101);

pub const PMU_EVENT_REMOTE_ACCESS: BitVector<16> = BitVector::<16>::new(0b0000000000110001);

pub const PMU_EVENT_LL_CACHE: BitVector<16> = BitVector::<16>::new(0b0000000000110010);

pub const PMU_EVENT_LL_CACHE_MISS: BitVector<16> = BitVector::<16>::new(0b0000000000110011);

pub const PMU_EVENT_DTLB_WALK: BitVector<16> = BitVector::<16>::new(0b0000000000110100);

pub const PMU_EVENT_L1D_CACHE_LMISS_RD: BitVector<16> = BitVector::<16>::new(0b0000000000111001);

pub const PMU_EVENT_L2D_CACHE_RD: BitVector<16> = BitVector::<16>::new(0b0000000001010000);

pub const PMU_EVENT_SAMPLE_POP: BitVector<16> = BitVector::<16>::new(0b0100000000000000);

pub const PMU_EVENT_SAMPLE_FEED: BitVector<16> = BitVector::<16>::new(0b0100000000000001);

pub const PMU_EVENT_SAMPLE_FILTRATE: BitVector<16> = BitVector::<16>::new(0b0100000000000010);

pub const PMU_EVENT_SAMPLE_COLLISION: BitVector<16> = BitVector::<16>::new(0b0100000000000011);

pub const PMU_EVENT_L2D_CACHE_LMISS_RD: BitVector<16> = BitVector::<16>::new(0b0100000000001001);

pub const PMU_EVENT_LDST_ALIGN_LAT: BitVector<16> = BitVector::<16>::new(0b0100000000100000);

pub const PMU_EVENT_SVE_PRED_EMPTY_SPEC: BitVector<16> = BitVector::<16>::new(0b1000000001110101);

pub const PMU_EVENT_SVE_PRED_PARTIAL_SPEC: BitVector<16> = BitVector::<16>::new(0b1000000001110111);

pub const PMU_EVENT_BRB_FILTRATE: BitVector<16> = BitVector::<16>::new(0b1000000100011111);

pub const PMU_EVENT_SAMPLE_WRAP: BitVector<16> = BitVector::<16>::new(0b1000000100100011);

pub const PMU_EVENT_SAMPLE_FEED_BR: BitVector<16> = BitVector::<16>::new(0b1000000100101010);

pub const PMU_EVENT_SAMPLE_FEED_LD: BitVector<16> = BitVector::<16>::new(0b1000000100101011);

pub const PMU_EVENT_SAMPLE_FEED_ST: BitVector<16> = BitVector::<16>::new(0b1000000100101100);

pub const PMU_EVENT_SAMPLE_FEED_OP: BitVector<16> = BitVector::<16>::new(0b1000000100101101);

pub const PMU_EVENT_SAMPLE_FEED_EVENT: BitVector<16> = BitVector::<16>::new(0b1000000100101110);

pub const PMU_EVENT_SAMPLE_FEED_LAT: BitVector<16> = BitVector::<16>::new(0b1000000100101111);

pub const PMU_EVENT_DSNP_HIT_RD: BitVector<16> = BitVector::<16>::new(0b1000000110010100);

pub const PMU_EVENT_L1D_CACHE_HITM_RD: BitVector<16> = BitVector::<16>::new(0b1000001000010100);

pub const PMU_EVENT_L2D_CACHE_HITM_RD: BitVector<16> = BitVector::<16>::new(0b1000001000010101);

pub const PMU_EVENT_L3D_CACHE_HITM_RD: BitVector<16> = BitVector::<16>::new(0b1000001000010110);

pub const PMU_EVENT_LL_CACHE_HITM_RD: BitVector<16> = BitVector::<16>::new(0b1000001000010111);

pub const PMU_EVENT_L1D_LFB_HIT_RD: BitVector<16> = BitVector::<16>::new(0b1000001001000100);

pub const PMU_EVENT_L2D_LFB_HIT_RD: BitVector<16> = BitVector::<16>::new(0b1000001001000101);

pub const PMU_EVENT_L3D_LFB_HIT_RD: BitVector<16> = BitVector::<16>::new(0b1000001001000110);

pub const PMU_EVENT_LL_LFB_HIT_RD: BitVector<16> = BitVector::<16>::new(0b1000001001000111);

/// SecurityState
///
/// Generated from the Sail sources at `src/v8_base.sail` L1492.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SecurityState {
    SS_NonSecure,
    SS_Root,
    SS_Realm,
    SS_Secure,
}

pub const M32_User: BitVector<5> = BitVector::<5>::new(0b10000);

pub const M32_FIQ: BitVector<5> = BitVector::<5>::new(0b10001);

pub const M32_IRQ: BitVector<5> = BitVector::<5>::new(0b10010);

pub const M32_Svc: BitVector<5> = BitVector::<5>::new(0b10011);

pub const M32_Monitor: BitVector<5> = BitVector::<5>::new(0b10110);

pub const M32_Abort: BitVector<5> = BitVector::<5>::new(0b10111);

pub const M32_Hyp: BitVector<5> = BitVector::<5>::new(0b11010);

pub const M32_Undef: BitVector<5> = BitVector::<5>::new(0b11011);

pub const M32_System: BitVector<5> = BitVector::<5>::new(0b11111);

pub const EL3: BitVector<2> = BitVector::<2>::new(0b11);

pub const EL2: BitVector<2> = BitVector::<2>::new(0b10);

pub const EL1: BitVector<2> = BitVector::<2>::new(0b01);

pub const EL0: BitVector<2> = BitVector::<2>::new(0b00);

/// ProcState
///
/// Generated from the Sail sources at `src/v8_base.sail` L1520-1553.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ProcState {
    pub N: BitVector<1>,
    pub Z: BitVector<1>,
    pub C: BitVector<1>,
    pub V: BitVector<1>,
    pub D: BitVector<1>,
    pub A: BitVector<1>,
    pub I: BitVector<1>,
    pub F: BitVector<1>,
    pub EXLOCK: BitVector<1>,
    pub PAN: BitVector<1>,
    pub UAO: BitVector<1>,
    pub DIT: BitVector<1>,
    pub TCO: BitVector<1>,
    pub PM: BitVector<1>,
    pub PPEND: BitVector<1>,
    pub BTYPE: BitVector<2>,
    pub ZA: BitVector<1>,
    pub SM: BitVector<1>,
    pub ALLINT: BitVector<1>,
    pub SS: BitVector<1>,
    pub IL: BitVector<1>,
    pub EL: BitVector<2>,
    pub nRW: BitVector<1>,
    pub SP: BitVector<1>,
    pub Q: BitVector<1>,
    pub GE: BitVector<4>,
    pub SSBS: BitVector<1>,
    pub IT: BitVector<8>,
    pub J: BitVector<1>,
    pub T: BitVector<1>,
    pub E: BitVector<1>,
    pub M: BitVector<5>,
}

/// PrivilegeLevel
///
/// Generated from the Sail sources at `src/v8_base.sail` L1561.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PrivilegeLevel {
    PL3,
    PL2,
    PL1,
    PL0,
}

/// InstrSet
///
/// Generated from the Sail sources at `src/v8_base.sail` L1563.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum InstrSet {
    InstrSet_A64,
    InstrSet_A32,
    InstrSet_T32,
}

/// DSBAlias
///
/// Generated from the Sail sources at `src/v8_base.sail` L1565.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum DSBAlias {
    DSBAlias_SSBB,
    DSBAlias_PSSBB,
    DSBAlias_DSB,
}

/// WFxType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1567.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum WFxType {
    WFxType_WFE,
    WFxType_WFI,
    WFxType_WFET,
    WFxType_WFIT,
}

/// ExceptionalOccurrenceTargetState
///
/// Generated from the Sail sources at `src/v8_base.sail` L1571-1575.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExceptionalOccurrenceTargetState {
    AArch32_NonDebugState,
    AArch64_NonDebugState,
    DebugState,
}

pub type PARTIDtype = BitVector<16>;

pub type PMGtype = BitVector<8>;

/// PARTIDspaceType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1589-1594.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PARTIDspaceType {
    PIdSpace_Secure,
    PIdSpace_Root,
    PIdSpace_Realm,
    PIdSpace_NonSecure,
}

/// MPAMinfo
///
/// Generated from the Sail sources at `src/v8_base.sail` L1596-1600.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MPAMinfo {
    pub mpam_sp: PARTIDspaceType,
    pub partid: PARTIDtype,
    pub pmg: PMGtype,
}

/// AccessType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1602-1617.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AccessType {
    AccessType_IFETCH,
    AccessType_GPR,
    AccessType_ASIMD,
    AccessType_SVE,
    AccessType_SME,
    AccessType_IC,
    AccessType_DC,
    AccessType_DCZero,
    AccessType_AT,
    AccessType_NV2,
    AccessType_SPE,
    AccessType_GCS,
    AccessType_GPTW,
    AccessType_TTW,
}

/// MemOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L1619.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MemOp {
    MemOp_LOAD,
    MemOp_STORE,
    MemOp_PREFETCH,
}

/// VARange
///
/// Generated from the Sail sources at `src/v8_base.sail` L1621.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VARange {
    VARange_LOWER,
    VARange_UPPER,
}

/// MemAtomicOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L1627-1639.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MemAtomicOp {
    MemAtomicOp_GCSSS1,
    MemAtomicOp_ADD,
    MemAtomicOp_BIC,
    MemAtomicOp_EOR,
    MemAtomicOp_ORR,
    MemAtomicOp_SMAX,
    MemAtomicOp_SMIN,
    MemAtomicOp_UMAX,
    MemAtomicOp_UMIN,
    MemAtomicOp_SWP,
    MemAtomicOp_CAS,
}

/// CacheOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L1641.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CacheOp {
    CacheOp_Clean,
    CacheOp_Invalidate,
    CacheOp_CleanInvalidate,
}

/// CacheOpScope
///
/// Generated from the Sail sources at `src/v8_base.sail` L1643-1653.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CacheOpScope {
    CacheOpScope_SetWay,
    CacheOpScope_PoU,
    CacheOpScope_PoC,
    CacheOpScope_PoE,
    CacheOpScope_PoP,
    CacheOpScope_PoDP,
    CacheOpScope_PoPA,
    CacheOpScope_ALLU,
    CacheOpScope_ALLUIS,
}

/// CacheType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1655-1660.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CacheType {
    CacheType_Data,
    CacheType_Tag,
    CacheType_Data_Tag,
    CacheType_Instruction,
}

/// CachePASpace
///
/// Generated from the Sail sources at `src/v8_base.sail` L1662-1670.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CachePASpace {
    CPAS_NonSecure,
    CPAS_Any,
    CPAS_RealmNonSecure,
    CPAS_Realm,
    CPAS_Root,
    CPAS_SecureNonSecure,
    CPAS_Secure,
}

/// AccessDescriptor
///
/// Generated from the Sail sources at `src/v8_base.sail` L1672-1706.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AccessDescriptor {
    pub acctype: AccessType,
    pub el: BitVector<2>,
    pub ss: SecurityState,
    pub acqsc: bool,
    pub acqpc: bool,
    pub relsc: bool,
    pub limitedordered: bool,
    pub exclusive: bool,
    pub atomicop: bool,
    pub modop: MemAtomicOp,
    pub nontemporal: bool,
    pub read: bool,
    pub write: bool,
    pub cacheop: CacheOp,
    pub opscope: CacheOpScope,
    pub cachetype: CacheType,
    pub pan: bool,
    pub transactional: bool,
    pub nonfault: bool,
    pub firstfault: bool,
    pub first: bool,
    pub contiguous: bool,
    pub streamingsve: bool,
    pub ls64: bool,
    pub mops: bool,
    pub rcw: bool,
    pub rcws: bool,
    pub toplevel: bool,
    pub varange: VARange,
    pub a32lsmd: bool,
    pub tagchecked: bool,
    pub tagaccess: bool,
    pub mpam: MPAMinfo,
}

/// MemType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1712.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MemType {
    MemType_Normal,
    MemType_Device,
}

/// DeviceType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1714-1719.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum DeviceType {
    DeviceType_GRE,
    DeviceType_nGRE,
    DeviceType_nGnRE,
    DeviceType_nGnRnE,
}

/// MemAttrHints
///
/// Generated from the Sail sources at `src/v8_base.sail` L1721.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MemAttrHints {
    pub attrs: BitVector<2>,
    pub hints: BitVector<2>,
    pub transient: bool,
}

pub const LOG2_TAG_GRANULE: i128 = 4;

pub const MemAttr_NC: BitVector<2> = BitVector::<2>::new(0b00);

pub const MemAttr_WT: BitVector<2> = BitVector::<2>::new(0b10);

pub const MemAttr_WB: BitVector<2> = BitVector::<2>::new(0b11);

pub const MemHint_No: BitVector<2> = BitVector::<2>::new(0b00);

pub const MemHint_WA: BitVector<2> = BitVector::<2>::new(0b01);

pub const MemHint_RA: BitVector<2> = BitVector::<2>::new(0b10);

pub const MemHint_RWA: BitVector<2> = BitVector::<2>::new(0b11);

/// Shareability
///
/// Generated from the Sail sources at `src/v8_base.sail` L1739.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Shareability {
    Shareability_NSH,
    Shareability_ISH,
    Shareability_OSH,
}

/// MemTagType
///
/// Generated from the Sail sources at `src/v8_base.sail` L1745-1749.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MemTagType {
    MemTag_Untagged,
    MemTag_AllocationTagged,
    MemTag_CanonicallyTagged,
}

/// MemoryAttributes
///
/// Generated from the Sail sources at `src/v8_base.sail` L1751-1760.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MemoryAttributes {
    pub memtype: MemType,
    pub device: DeviceType,
    pub inner: MemAttrHints,
    pub outer: MemAttrHints,
    pub shareability: Shareability,
    pub tags: MemTagType,
    pub notagaccess: bool,
    pub xs: BitVector<1>,
}

/// PASpace
///
/// Generated from the Sail sources at `src/v8_base.sail` L1766.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PASpace {
    PAS_NonSecure,
    PAS_Secure,
    PAS_Root,
    PAS_Realm,
}

/// FullAddress
///
/// Generated from the Sail sources at `src/v8_base.sail` L1772.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FullAddress {
    pub paspace: PASpace,
    pub address: BitVector<56>,
}

/// GPCF
///
/// Generated from the Sail sources at `src/v8_base.sail` L1778.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum GPCF {
    GPCF_None,
    GPCF_AddressSize,
    GPCF_Walk,
    GPCF_EABT,
    GPCF_Fail,
}

/// GPCFRecord
///
/// Generated from the Sail sources at `src/v8_base.sail` L1780.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GPCFRecord {
    pub gpf: GPCF,
    pub level: i128,
}

/// Fault
///
/// Generated from the Sail sources at `src/v8_base.sail` L1782-1807.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Fault {
    Fault_None,
    Fault_AccessFlag,
    Fault_Alignment,
    Fault_Background,
    Fault_Domain,
    Fault_Permission,
    Fault_Translation,
    Fault_AddressSize,
    Fault_SyncExternal,
    Fault_SyncExternalOnWalk,
    Fault_SyncParity,
    Fault_SyncParityOnWalk,
    Fault_GPCFOnWalk,
    Fault_GPCFOnOutput,
    Fault_AsyncParity,
    Fault_AsyncExternal,
    Fault_TagCheck,
    Fault_Debug,
    Fault_TLBConflict,
    Fault_BranchTarget,
    Fault_HWUpdateAccessFlag,
    Fault_Lockdown,
    Fault_Exclusive,
    Fault_ICacheMaint,
}

/// ErrorState
///
/// Generated from the Sail sources at `src/v8_base.sail` L1809-1817.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ErrorState {
    ErrorState_UC,
    ErrorState_UEU,
    ErrorState_UEO,
    ErrorState_UER,
    ErrorState_CE,
    ErrorState_Uncategorized,
    ErrorState_IMPDEF,
}

/// FaultRecord
///
/// Generated from the Sail sources at `src/v8_base.sail` L1819-1840.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct FaultRecord {
    pub statuscode: Fault,
    pub access: AccessDescriptor,
    pub ipaddress: FullAddress,
    pub gpcf: GPCFRecord,
    pub paddress: FullAddress,
    pub gpcfs2walk: bool,
    pub s2fs1walk: bool,
    pub write: bool,
    pub s1tagnotdata: bool,
    pub tagaccess: bool,
    pub level: i128,
    pub extflag: BitVector<1>,
    pub secondstage: bool,
    pub assuredonly: bool,
    pub toplevel: bool,
    pub overlay: bool,
    pub dirtybit: bool,
    pub domain: BitVector<4>,
    pub merrorstate: ErrorState,
    pub debugmoe: BitVector<4>,
}

/// PhysMemRetStatus
///
/// Generated from the Sail sources at `src/v8_base.sail` L1846-1851.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PhysMemRetStatus {
    pub statuscode: Fault,
    pub extflag: BitVector<1>,
    pub merrorstate: ErrorState,
    pub store64bstatus: BitVector<64>,
}

/// Permissions
///
/// Generated from the Sail sources at `src/v8_base.sail` L1857-1877.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Permissions {
    pub ap_table: BitVector<2>,
    pub xn_table: BitVector<1>,
    pub pxn_table: BitVector<1>,
    pub uxn_table: BitVector<1>,
    pub ap: BitVector<3>,
    pub xn: BitVector<1>,
    pub uxn: BitVector<1>,
    pub pxn: BitVector<1>,
    pub ppi: BitVector<4>,
    pub upi: BitVector<4>,
    pub ndirty: BitVector<1>,
    pub s2pi: BitVector<4>,
    pub s2dirty: BitVector<1>,
    pub po_index: BitVector<4>,
    pub s2po_index: BitVector<4>,
    pub s2ap: BitVector<2>,
    pub s2tag_na: BitVector<1>,
    pub s2xnx: BitVector<1>,
    pub s2xn: BitVector<1>,
}

/// S1AccessControls
///
/// Generated from the Sail sources at `src/v8_base.sail` L1879-1889.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S1AccessControls {
    pub r: BitVector<1>,
    pub w: BitVector<1>,
    pub x: BitVector<1>,
    pub gcs: BitVector<1>,
    pub overlay: bool,
    pub or: BitVector<1>,
    pub ow: BitVector<1>,
    pub ox: BitVector<1>,
    pub wxn: BitVector<1>,
}

/// S2AccessControls
///
/// Generated from the Sail sources at `src/v8_base.sail` L1891-1909.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S2AccessControls {
    pub r: BitVector<1>,
    pub w: BitVector<1>,
    pub x: BitVector<1>,
    pub r_rcw: BitVector<1>,
    pub w_rcw: BitVector<1>,
    pub r_mmu: BitVector<1>,
    pub w_mmu: BitVector<1>,
    pub toplevel0: BitVector<1>,
    pub toplevel1: BitVector<1>,
    pub overlay: bool,
    pub or: BitVector<1>,
    pub ow: BitVector<1>,
    pub ox: BitVector<1>,
    pub or_rcw: BitVector<1>,
    pub ow_rcw: BitVector<1>,
    pub or_mmu: BitVector<1>,
    pub ow_mmu: BitVector<1>,
}

/// MBReqDomain
///
/// Generated from the Sail sources at `src/v8_base.sail` L1911-1916.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MBReqDomain {
    MBReqDomain_Nonshareable,
    MBReqDomain_InnerShareable,
    MBReqDomain_OuterShareable,
    MBReqDomain_FullSystem,
}

/// MBReqTypes
///
/// Generated from the Sail sources at `src/v8_base.sail` L1918.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MBReqTypes {
    MBReqTypes_Reads,
    MBReqTypes_Writes,
    MBReqTypes_All,
}

/// PrefetchHint
///
/// Generated from the Sail sources at `src/v8_base.sail` L1920.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PrefetchHint {
    Prefetch_READ,
    Prefetch_WRITE,
    Prefetch_EXEC,
}

/// Unpredictable
///
/// Generated from the Sail sources at `src/v8_base.sail` L1922-2002.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Unpredictable {
    Unpredictable_VMSR,
    Unpredictable_WBOVERLAPLD,
    Unpredictable_WBOVERLAPST,
    Unpredictable_LDPOVERLAP,
    Unpredictable_BASEOVERLAP,
    Unpredictable_DATAOVERLAP,
    Unpredictable_DEVPAGE2,
    Unpredictable_INSTRDEVICE,
    Unpredictable_RESCPACR,
    Unpredictable_RESMAIR,
    Unpredictable_S1CTAGGED,
    Unpredictable_S2RESMEMATTR,
    Unpredictable_RESTEXCB,
    Unpredictable_RESPRRR,
    Unpredictable_RESDACR,
    Unpredictable_RESVTCRS,
    Unpredictable_RESTnSZ,
    Unpredictable_RESTCF,
    Unpredictable_DEVICETAGSTORE,
    Unpredictable_OORTnSZ,
    Unpredictable_LARGEIPA,
    Unpredictable_ESRCONDPASS,
    Unpredictable_ILZEROIT,
    Unpredictable_ILZEROT,
    Unpredictable_BPVECTORCATCHPRI,
    Unpredictable_VCMATCHHALF,
    Unpredictable_VCMATCHDAPA,
    Unpredictable_WPMASKANDBAS,
    Unpredictable_WPBASCONTIGUOUS,
    Unpredictable_RESWPMASK,
    Unpredictable_WPMASKEDBITS,
    Unpredictable_RESBPWPCTRL,
    Unpredictable_BPNOTIMPL,
    Unpredictable_RESBPTYPE,
    Unpredictable_RESMDSELR,
    Unpredictable_BPNOTCTXCMP,
    Unpredictable_BPMATCHHALF,
    Unpredictable_BPMISMATCHHALF,
    Unpredictable_BPLINKINGDISABLED,
    Unpredictable_RESBPMASK,
    Unpredictable_BPMASK,
    Unpredictable_BPMASKEDBITS,
    Unpredictable_BPLINKEDADDRMATCH,
    Unpredictable_RESTARTALIGNPC,
    Unpredictable_RESTARTZEROUPPERPC,
    Unpredictable_ZEROUPPER,
    Unpredictable_ERETZEROUPPERPC,
    Unpredictable_A32FORCEALIGNPC,
    Unpredictable_SMD,
    Unpredictable_NONFAULT,
    Unpredictable_SVEZEROUPPER,
    Unpredictable_SVELDNFDATA,
    Unpredictable_SVELDNFZERO,
    Unpredictable_CHECKSPNONEACTIVE,
    Unpredictable_SMEZEROUPPER,
    Unpredictable_NVNV1,
    Unpredictable_Shareability,
    Unpredictable_AFUPDATE,
    Unpredictable_DBUPDATE,
    Unpredictable_IESBinDebug,
    Unpredictable_BADPMSFCR,
    Unpredictable_ZEROBTYPE,
    Unpredictable_EL2TIMESTAMP,
    Unpredictable_EL1TIMESTAMP,
    Unpredictable_RESERVEDNSxB,
    Unpredictable_WFxTDEBUG,
    Unpredictable_LS64UNSUPPORTED,
    Unpredictable_MISALIGNEDATOMIC,
    Unpredictable_CLEARERRITEZERO,
    Unpredictable_ALUEXCEPTIONRETURN,
    Unpredictable_IGNORETRAPINDEBUG,
    Unpredictable_DBGxVR_RESS,
    Unpredictable_PMUEVENTCOUNTER,
    Unpredictable_PMSCR_PCT,
    Unpredictable_CounterReservedForEL2,
    Unpredictable_BRBFILTRATE,
    Unpredictable_MOPSOVERLAP31,
    Unpredictable_STOREONLYTAGCHECKEDCAS,
    Unpredictable_RESTC,
}

/// Constraint
///
/// Generated from the Sail sources at `src/v8_base.sail` L2004-2034.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Constraint {
    Constraint_NONE,
    Constraint_UNKNOWN,
    Constraint_UNDEF,
    Constraint_UNDEFEL0,
    Constraint_NOP,
    Constraint_TRUE,
    Constraint_FALSE,
    Constraint_DISABLED,
    Constraint_UNCOND,
    Constraint_COND,
    Constraint_ADDITIONAL_DECODE,
    Constraint_WBSUPPRESS,
    Constraint_FAULT,
    Constraint_LIMITED_ATOMICITY,
    Constraint_NVNV1_00,
    Constraint_NVNV1_01,
    Constraint_NVNV1_11,
    Constraint_EL1TIMESTAMP,
    Constraint_EL2TIMESTAMP,
    Constraint_OSH,
    Constraint_ISH,
    Constraint_NSH,
    Constraint_NC,
    Constraint_WT,
    Constraint_WB,
    Constraint_FORCE,
    Constraint_FORCENOSLCHECK,
    Constraint_MAPTOALLOCATED,
    Constraint_PMSCR_PCT_VIRT,
}

/// CacheRecord
///
/// Generated from the Sail sources at `src/v8_base.sail` L2040-2059.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CacheRecord {
    pub acctype: AccessType,
    pub cacheop: CacheOp,
    pub opscope: CacheOpScope,
    pub cachetype: CacheType,
    pub regval: BitVector<64>,
    pub paddress: FullAddress,
    pub vaddress: BitVector<64>,
    pub setnum: i128,
    pub waynum: i128,
    pub level: i128,
    pub shareability: Shareability,
    pub translated: bool,
    pub is_vmid_valid: bool,
    pub vmid: BitVector<16>,
    pub is_asid_valid: bool,
    pub asid: BitVector<16>,
    pub security: SecurityState,
    pub cpas: CachePASpace,
}

/// RestrictType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2061-2066.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum RestrictType {
    RestrictType_DataValue,
    RestrictType_ControlFlow,
    RestrictType_CachePrefetch,
    RestrictType_Other,
}

/// ExecutionCntxt
///
/// Generated from the Sail sources at `src/v8_base.sail` L2068-2078.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ExecutionCntxt {
    pub is_vmid_valid: bool,
    pub all_vmid: bool,
    pub vmid: BitVector<16>,
    pub is_asid_valid: bool,
    pub all_asid: bool,
    pub asid: BitVector<16>,
    pub target_el: BitVector<2>,
    pub security: SecurityState,
    pub restriction: RestrictType,
}

/// FPExc
///
/// Generated from the Sail sources at `src/v8_base.sail` L2080-2087.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FPExc {
    FPExc_InvalidOp,
    FPExc_DivideByZero,
    FPExc_Overflow,
    FPExc_Underflow,
    FPExc_Inexact,
    FPExc_InputDenorm,
}

/// FPRounding
///
/// Generated from the Sail sources at `src/v8_base.sail` L2089-2096.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FPRounding {
    FPRounding_TIEEVEN,
    FPRounding_POSINF,
    FPRounding_NEGINF,
    FPRounding_ZERO,
    FPRounding_TIEAWAY,
    FPRounding_ODD,
}

/// FPType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2098-2105.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FPType {
    FPType_Zero,
    FPType_Denormal,
    FPType_Nonzero,
    FPType_Infinity,
    FPType_QNaN,
    FPType_SNaN,
}

/// read_gpr
///
/// Generated from the Sail sources at `src/v8_base.sail` L2143-2177.
pub fn read_gpr(core_ctx: &mut Core, n: i128) -> BitVector<64> {
    match n {
        l__9997 if { (l__9997 == 0) } => core_ctx.R0,
        l__9998 if { (l__9998 == 1) } => core_ctx.R1,
        l__9999 if { (l__9999 == 2) } => core_ctx.R2,
        l__10000 if { (l__10000 == 3) } => core_ctx.R3,
        l__10001 if { (l__10001 == 4) } => core_ctx.R4,
        l__10002 if { (l__10002 == 5) } => core_ctx.R5,
        l__10003 if { (l__10003 == 6) } => core_ctx.R6,
        l__10004 if { (l__10004 == 7) } => core_ctx.R7,
        l__10005 if { (l__10005 == 8) } => core_ctx.R8,
        l__10006 if { (l__10006 == 9) } => core_ctx.R9,
        l__10007 if { (l__10007 == 10) } => core_ctx.R10,
        l__10008 if { (l__10008 == 11) } => core_ctx.R11,
        l__10009 if { (l__10009 == 12) } => core_ctx.R12,
        l__10010 if { (l__10010 == 13) } => core_ctx.R13,
        l__10011 if { (l__10011 == 14) } => core_ctx.R14,
        l__10012 if { (l__10012 == 15) } => core_ctx.R15,
        l__10013 if { (l__10013 == 16) } => core_ctx.R16,
        l__10014 if { (l__10014 == 17) } => core_ctx.R17,
        l__10015 if { (l__10015 == 18) } => core_ctx.R18,
        l__10016 if { (l__10016 == 19) } => core_ctx.R19,
        l__10017 if { (l__10017 == 20) } => core_ctx.R20,
        l__10018 if { (l__10018 == 21) } => core_ctx.R21,
        l__10019 if { (l__10019 == 22) } => core_ctx.R22,
        l__10020 if { (l__10020 == 23) } => core_ctx.R23,
        l__10021 if { (l__10021 == 24) } => core_ctx.R24,
        l__10022 if { (l__10022 == 25) } => core_ctx.R25,
        l__10023 if { (l__10023 == 26) } => core_ctx.R26,
        l__10024 if { (l__10024 == 27) } => core_ctx.R27,
        l__10025 if { (l__10025 == 28) } => core_ctx.R28,
        l__10026 if { (l__10026 == 29) } => core_ctx.R29,
        _ => core_ctx.R30,
        _ => {
            panic!("Unreachable code")
        }
    }
}

/// write_gpr
///
/// Generated from the Sail sources at `src/v8_base.sail` L2220-2254.
pub fn write_gpr(core_ctx: &mut Core, n: i128, v: BitVector<64>) {
    match n {
        l__9937 if { (l__9937 == 0) } => core_ctx.R0 = v,
        l__9938 if { (l__9938 == 1) } => core_ctx.R1 = v,
        l__9939 if { (l__9939 == 2) } => core_ctx.R2 = v,
        l__9940 if { (l__9940 == 3) } => core_ctx.R3 = v,
        l__9941 if { (l__9941 == 4) } => core_ctx.R4 = v,
        l__9942 if { (l__9942 == 5) } => core_ctx.R5 = v,
        l__9943 if { (l__9943 == 6) } => core_ctx.R6 = v,
        l__9944 if { (l__9944 == 7) } => core_ctx.R7 = v,
        l__9945 if { (l__9945 == 8) } => core_ctx.R8 = v,
        l__9946 if { (l__9946 == 9) } => core_ctx.R9 = v,
        l__9947 if { (l__9947 == 10) } => core_ctx.R10 = v,
        l__9948 if { (l__9948 == 11) } => core_ctx.R11 = v,
        l__9949 if { (l__9949 == 12) } => core_ctx.R12 = v,
        l__9950 if { (l__9950 == 13) } => core_ctx.R13 = v,
        l__9951 if { (l__9951 == 14) } => core_ctx.R14 = v,
        l__9952 if { (l__9952 == 15) } => core_ctx.R15 = v,
        l__9953 if { (l__9953 == 16) } => core_ctx.R16 = v,
        l__9954 if { (l__9954 == 17) } => core_ctx.R17 = v,
        l__9955 if { (l__9955 == 18) } => core_ctx.R18 = v,
        l__9956 if { (l__9956 == 19) } => core_ctx.R19 = v,
        l__9957 if { (l__9957 == 20) } => core_ctx.R20 = v,
        l__9958 if { (l__9958 == 21) } => core_ctx.R21 = v,
        l__9959 if { (l__9959 == 22) } => core_ctx.R22 = v,
        l__9960 if { (l__9960 == 23) } => core_ctx.R23 = v,
        l__9961 if { (l__9961 == 24) } => core_ctx.R24 = v,
        l__9962 if { (l__9962 == 25) } => core_ctx.R25 = v,
        l__9963 if { (l__9963 == 26) } => core_ctx.R26 = v,
        l__9964 if { (l__9964 == 27) } => core_ctx.R27 = v,
        l__9965 if { (l__9965 == 28) } => core_ctx.R28 = v,
        l__9966 if { (l__9966 == 29) } => core_ctx.R29 = v,
        _ => core_ctx.R30 = v,
        _ => {
            panic!("Unreachable code")
        }
    }
}

/// get_R
///
/// Generated from the Sail sources at `src/v8_base.sail` L2321.
pub fn get_R(core_ctx: &mut Core, n: i128) -> BitVector<64> {
    read_gpr(core_ctx, n)
}

/// set_R
///
/// Generated from the Sail sources at `src/v8_base.sail` L2325.
pub fn set_R(core_ctx: &mut Core, n: i128, v: BitVector<64>) {
    write_gpr(core_ctx, n, v)
}

/// BranchType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2393-2405.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum BranchType {
    BranchType_DIRCALL,
    BranchType_INDCALL,
    BranchType_ERET,
    BranchType_DBGEXIT,
    BranchType_RET,
    BranchType_DIR,
    BranchType_INDIR,
    BranchType_EXCEPTION,
    BranchType_TMFAIL,
    BranchType_RESET,
    BranchType_UNKNOWN,
}

/// InterruptID
///
/// Generated from the Sail sources at `src/v8_base.sail` L2409-2423.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum InterruptID {
    InterruptID_PMUIRQ,
    InterruptID_COMMIRQ,
    InterruptID_CTIIRQ,
    InterruptID_COMMRX,
    InterruptID_COMMTX,
    InterruptID_CNTP,
    InterruptID_CNTHP,
    InterruptID_CNTHPS,
    InterruptID_CNTPS,
    InterruptID_CNTV,
    InterruptID_CNTHV,
    InterruptID_CNTHVS,
    InterruptID_PMBIRQ,
}

pub const DefaultPARTID: PARTIDtype = integer_subrange(0, 15, 0);

pub const DefaultPMG: PMGtype = integer_subrange(0, 7, 0);

/// Exception
///
/// Generated from the Sail sources at `src/v8_base.sail` L2429-2471.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Exception {
    Exception_Uncategorized,
    Exception_WFxTrap,
    Exception_CP15RTTrap,
    Exception_CP15RRTTrap,
    Exception_CP14RTTrap,
    Exception_CP14DTTrap,
    Exception_CP14RRTTrap,
    Exception_AdvSIMDFPAccessTrap,
    Exception_FPIDTrap,
    Exception_LDST64BTrap,
    Exception_PACTrap,
    Exception_IllegalState,
    Exception_SupervisorCall,
    Exception_HypervisorCall,
    Exception_MonitorCall,
    Exception_SystemRegisterTrap,
    Exception_ERetTrap,
    Exception_InstructionAbort,
    Exception_PCAlignment,
    Exception_DataAbort,
    Exception_NV2DataAbort,
    Exception_PACFail,
    Exception_SPAlignment,
    Exception_FPTrappedException,
    Exception_SError,
    Exception_Breakpoint,
    Exception_SoftwareStep,
    Exception_Watchpoint,
    Exception_NV2Watchpoint,
    Exception_SoftwareBreakpoint,
    Exception_VectorCatch,
    Exception_IRQ,
    Exception_SVEAccessTrap,
    Exception_SMEAccessTrap,
    Exception_TSTARTAccessTrap,
    Exception_GPC,
    Exception_BranchTarget,
    Exception_MemCpyMemSet,
    Exception_GCSFail,
    Exception_SystemRegister128Trap,
    Exception_FIQ,
}

/// ExceptionRecord
///
/// Generated from the Sail sources at `src/v8_base.sail` L2473-2484.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct ExceptionRecord {
    pub exceptype: Exception,
    pub syndrome: BitVector<25>,
    pub syndrome2: BitVector<24>,
    pub paddress: FullAddress,
    pub vaddress: BitVector<64>,
    pub ipavalid: bool,
    pub pavalid: bool,
    pub NS: BitVector<1>,
    pub ipaddress: BitVector<56>,
    pub trappedsyscallinst: bool,
}

/// TranslationStage
///
/// Generated from the Sail sources at `src/v8_base.sail` L2486.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TranslationStage {
    TranslationStage_1,
    TranslationStage_12,
}

/// ATAccess
///
/// Generated from the Sail sources at `src/v8_base.sail` L2488-2493.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ATAccess {
    ATAccess_Read,
    ATAccess_Write,
    ATAccess_ReadPAN,
    ATAccess_WritePAN,
}

/// Regime
///
/// Generated from the Sail sources at `src/v8_base.sail` L2495.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Regime {
    Regime_EL3,
    Regime_EL30,
    Regime_EL2,
    Regime_EL20,
    Regime_EL10,
}

/// TGx
///
/// Generated from the Sail sources at `src/v8_base.sail` L2497.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TGx {
    TGx_4KB,
    TGx_16KB,
    TGx_64KB,
}

/// DescriptorType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2499-2503.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum DescriptorType {
    DescriptorType_Table,
    DescriptorType_Leaf,
    DescriptorType_Invalid,
}

pub const Domain_NoAccess: BitVector<2> = BitVector::<2>::new(0b00);

pub const Domain_Client: BitVector<2> = BitVector::<2>::new(0b01);

/// SDFType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2509-2516.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SDFType {
    SDFType_Table,
    SDFType_Invalid,
    SDFType_Supersection,
    SDFType_Section,
    SDFType_LargePage,
    SDFType_SmallPage,
}

/// TTWState
///
/// Generated from the Sail sources at `src/v8_base.sail` L2522-2536.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TTWState {
    pub istable: bool,
    pub level: i128,
    pub baseaddress: FullAddress,
    pub contiguous: BitVector<1>,
    pub s1assured: bool,
    pub s2assuredonly: BitVector<1>,
    pub disch: BitVector<1>,
    pub nG: BitVector<1>,
    pub guardedpage: BitVector<1>,
    pub sdftype: SDFType,
    pub domain: BitVector<4>,
    pub memattrs: MemoryAttributes,
    pub permissions: Permissions,
}

/// S1TTWParams
///
/// Generated from the Sail sources at `src/v8_base.sail` L2542-2583.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S1TTWParams {
    pub ha: BitVector<1>,
    pub hd: BitVector<1>,
    pub tbi: BitVector<1>,
    pub tbid: BitVector<1>,
    pub nfd: BitVector<1>,
    pub e0pd: BitVector<1>,
    pub d128: BitVector<1>,
    pub aie: BitVector<1>,
    pub mair2: MAIRType,
    pub ds: BitVector<1>,
    pub ps: BitVector<3>,
    pub txsz: BitVector<6>,
    pub epan: BitVector<1>,
    pub dct: BitVector<1>,
    pub nv1: BitVector<1>,
    pub cmow: BitVector<1>,
    pub pnch: BitVector<1>,
    pub disch: BitVector<1>,
    pub haft: BitVector<1>,
    pub mtx: BitVector<1>,
    pub skl: BitVector<2>,
    pub pie: BitVector<1>,
    pub pir: S1PIRType,
    pub pire0: S1PIRType,
    pub emec: BitVector<1>,
    pub amec: BitVector<1>,
    pub t0sz: BitVector<3>,
    pub t1sz: BitVector<3>,
    pub uwxn: BitVector<1>,
    pub tgx: TGx,
    pub irgn: BitVector<2>,
    pub orgn: BitVector<2>,
    pub sh: BitVector<2>,
    pub hpd: BitVector<1>,
    pub ee: BitVector<1>,
    pub wxn: BitVector<1>,
    pub ntlsmd: BitVector<1>,
    pub dc: BitVector<1>,
    pub sif: BitVector<1>,
    pub mair: MAIRType,
}

/// S2TTWParams
///
/// Generated from the Sail sources at `src/v8_base.sail` L2585-2617.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct S2TTWParams {
    pub ha: BitVector<1>,
    pub hd: BitVector<1>,
    pub sl2: BitVector<1>,
    pub ds: BitVector<1>,
    pub d128: BitVector<1>,
    pub sw: BitVector<1>,
    pub nsw: BitVector<1>,
    pub sa: BitVector<1>,
    pub nsa: BitVector<1>,
    pub ps: BitVector<3>,
    pub txsz: BitVector<6>,
    pub fwb: BitVector<1>,
    pub cmow: BitVector<1>,
    pub skl: BitVector<2>,
    pub s2pie: BitVector<1>,
    pub s2pir: S2PIRType,
    pub tl0: BitVector<1>,
    pub tl1: BitVector<1>,
    pub assuredonly: BitVector<1>,
    pub haft: BitVector<1>,
    pub emec: BitVector<1>,
    pub s: BitVector<1>,
    pub t0sz: BitVector<4>,
    pub tgx: TGx,
    pub sl0: BitVector<2>,
    pub irgn: BitVector<2>,
    pub orgn: BitVector<2>,
    pub sh: BitVector<2>,
    pub ee: BitVector<1>,
    pub ptw: BitVector<1>,
    pub vm: BitVector<1>,
}

/// TLBContext
///
/// Generated from the Sail sources at `src/v8_base.sail` L2619-2635.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLBContext {
    pub ss: SecurityState,
    pub regime: Regime,
    pub vmid: BitVector<16>,
    pub asid: BitVector<16>,
    pub nG: BitVector<1>,
    pub ipaspace: PASpace,
    pub includes_s1_name: bool,
    pub includes_s2_name: bool,
    pub includes_gpt_name: bool,
    pub ia: BitVector<64>,
    pub tg: TGx,
    pub cnp: BitVector<1>,
    pub level: i128,
    pub isd128: bool,
    pub xs: BitVector<1>,
}

/// TLBRecord
///
/// Generated from the Sail sources at `src/v8_base.sail` L2637-2644.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLBRecord {
    pub context: TLBContext,
    pub walkstate: TTWState,
    pub blocksize: i128,
    pub contigsize: i128,
    pub s1descriptor: BitVector<128>,
    pub s2descriptor: BitVector<128>,
}

/// AddressDescriptor
///
/// Generated from the Sail sources at `src/v8_base.sail` L2646-2655.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct AddressDescriptor {
    pub fault: FaultRecord,
    pub memattrs: MemoryAttributes,
    pub paddress: FullAddress,
    pub tlbcontext: TLBContext,
    pub s1assured: bool,
    pub s2fs1mro: bool,
    pub mecid: BitVector<16>,
    pub vaddress: BitVector<64>,
}

/// TranslationInfo
///
/// Generated from the Sail sources at `src/v8_base.sail` L2661-2671.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TranslationInfo {
    pub regime: Regime,
    pub vmid: Option<BitVector<16>>,
    pub asid: Option<BitVector<16>>,
    pub va: BitVector<64>,
    pub s1level: Option<i128>,
    pub s2info: Option<(BitVector<64>, i128)>,
    pub s1params: Option<S1TTWParams>,
    pub s2params: Option<S2TTWParams>,
    pub memattrs: MemoryAttributes,
}

/// TranslationStartInfo
///
/// Generated from the Sail sources at `src/v8_base.sail` L2673-2682.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TranslationStartInfo {
    pub ss: SecurityState,
    pub regime: Regime,
    pub vmid: BitVector<16>,
    pub asid: BitVector<16>,
    pub va: BitVector<64>,
    pub cnp: BitVector<1>,
    pub accdesc: AccessDescriptor,
    pub size: i128,
}

pub const FINAL_LEVEL: i128 = 3;

/// SVECmp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2698.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SVECmp {
    Cmp_EQ,
    Cmp_NE,
    Cmp_GE,
    Cmp_GT,
    Cmp_LT,
    Cmp_LE,
    Cmp_UN,
}

pub const MAX_VL: i128 = 2048;

pub const MAX_PL: i128 = 256;

pub const ZT0_LEN: i128 = 512;

/// SMEExceptionType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2716-2722.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SMEExceptionType {
    SMEExceptionType_AccessTrap,
    SMEExceptionType_Streaming,
    SMEExceptionType_NotStreaming,
    SMEExceptionType_InactiveZA,
    SMEExceptionType_InaccessibleZT0,
}

/// GCSInstruction
///
/// Generated from the Sail sources at `src/v8_base.sail` L2728-2737.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum GCSInstruction {
    GCSInstType_PRET,
    GCSInstType_POPM,
    GCSInstType_PRETAA,
    GCSInstType_PRETAB,
    GCSInstType_SS1,
    GCSInstType_SS2,
    GCSInstType_POPCX,
    GCSInstType_POPX,
}

/// MOPSStage
///
/// Generated from the Sail sources at `src/v8_base.sail` L2739.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MOPSStage {
    MOPSStage_Prologue,
    MOPSStage_Main,
    MOPSStage_Epilogue,
}

pub const DEFAULT_MECID: BitVector<16> = Zeros::<16>(16);

/// TMState
///
/// Generated from the Sail sources at `src/v8_base.sail` L2787-2805.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TMState {
    pub depth: i128,
    pub Rt: i128,
    pub nPC: BitVector<64>,
    pub X: [BitVector<64>; (31 as usize)],
    pub Z: [BitVector<MAX_VL>; (32 as usize)],
    pub P: [BitVector<MAX_PL>; (16 as usize)],
    pub FFR: BitVector<MAX_PL>,
    pub SP: BitVector<64>,
    pub FPCR: BitVector<64>,
    pub FPSR: BitVector<64>,
    pub ICC_PMR_EL1: BitVector<64>,
    pub GCSPR_ELx: BitVector<64>,
    pub nzcv: BitVector<4>,
    pub D: BitVector<1>,
    pub A: BitVector<1>,
    pub I: BitVector<1>,
    pub F: BitVector<1>,
}

/// TMFailure
///
/// Generated from the Sail sources at `src/v8_base.sail` L2809-2818.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TMFailure {
    TMFailure_CNCL,
    TMFailure_DBG,
    TMFailure_ERR,
    TMFailure_NEST,
    TMFailure_SIZE,
    TMFailure_MEM,
    TMFailure_TRIVIAL,
    TMFailure_IMP,
}

/// PGSe
///
/// Generated from the Sail sources at `src/v8_base.sail` L2820.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PGSe {
    PGS_4KB,
    PGS_16KB,
    PGS_64KB,
}

pub const GPT_NoAccess: BitVector<4> = BitVector::<4>::new(0b0000);

pub const GPT_Table: BitVector<4> = BitVector::<4>::new(0b0011);

pub const GPT_Block: BitVector<4> = BitVector::<4>::new(0b0001);

pub const GPT_Contig: BitVector<4> = BitVector::<4>::new(0b0001);

pub const GPT_Secure: BitVector<4> = BitVector::<4>::new(0b1000);

pub const GPT_NonSecure: BitVector<4> = BitVector::<4>::new(0b1001);

pub const GPT_Root: BitVector<4> = BitVector::<4>::new(0b1010);

pub const GPT_Realm: BitVector<4> = BitVector::<4>::new(0b1011);

pub const GPT_Any: BitVector<4> = BitVector::<4>::new(0b1111);

pub const GPTRange_4KB: i128 = 12;

pub const GPTRange_16KB: i128 = 14;

pub const GPTRange_64KB: i128 = 16;

pub const GPTRange_2MB: i128 = 21;

pub const GPTRange_32MB: i128 = 25;

pub const GPTRange_512MB: i128 = 29;

pub const GPTRange_1GB: i128 = 30;

pub const GPTRange_16GB: i128 = 34;

pub const GPTRange_64GB: i128 = 36;

pub const GPTRange_512GB: i128 = 39;

/// GPTTable
///
/// Generated from the Sail sources at `src/v8_base.sail` L2860.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GPTTable {
    pub address: BitVector<56>,
}

/// GPTEntry
///
/// Generated from the Sail sources at `src/v8_base.sail` L2866-2872.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GPTEntry {
    pub gpi: BitVector<4>,
    pub size: i128,
    pub contig_size: i128,
    pub level: i128,
    pub pa: BitVector<56>,
}

pub const SPEMaxAddrs: i128 = 32;

pub const SPEMaxCounters: i128 = 32;

pub const SPEMaxRecordSize: i128 = 64;

/// TimeStamp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2884-2890.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TimeStamp {
    TimeStamp_None,
    TimeStamp_CoreSight,
    TimeStamp_Physical,
    TimeStamp_OffsetPhysical,
    TimeStamp_Virtual,
}

/// OpType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2892-2898.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum OpType {
    OpType_Load,
    OpType_Store,
    OpType_LoadAtomic,
    OpType_Branch,
    OpType_Other,
}

pub const SPEAddrPosPCVirtual: i128 = 0;

pub const SPEAddrPosBranchTarget: i128 = 1;

pub const SPEAddrPosDataVirtual: i128 = 2;

pub const SPEAddrPosDataPhysical: i128 = 3;

pub const SPEAddrPosPrevBranchTarget: i128 = 4;

pub const SPECounterPosTotalLatency: i128 = 0;

pub const SPECounterPosIssueLatency: i128 = 1;

pub const SPECounterPosTranslationLatency: i128 = 2;

/// CountOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2960.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CountOp {
    CountOp_CLZ,
    CountOp_CLS,
    CountOp_CNT,
}

/// ExtendType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2962-2971.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExtendType {
    ExtendType_SXTB,
    ExtendType_SXTH,
    ExtendType_SXTW,
    ExtendType_SXTX,
    ExtendType_UXTB,
    ExtendType_UXTH,
    ExtendType_UXTW,
    ExtendType_UXTX,
}

/// FPMaxMinOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2973-2978.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FPMaxMinOp {
    FPMaxMinOp_MAX,
    FPMaxMinOp_MIN,
    FPMaxMinOp_MAXNUM,
    FPMaxMinOp_MINNUM,
}

/// FPUnaryOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2980.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FPUnaryOp {
    FPUnaryOp_ABS,
    FPUnaryOp_MOV,
    FPUnaryOp_NEG,
    FPUnaryOp_SQRT,
}

/// FPConvOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2982-2988.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FPConvOp {
    FPConvOp_CVT_FtoI,
    FPConvOp_CVT_ItoF,
    FPConvOp_MOV_FtoI,
    FPConvOp_MOV_ItoF,
    FPConvOp_CVT_FtoI_JS,
}

/// MoveWideOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2990.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MoveWideOp {
    MoveWideOp_N,
    MoveWideOp_Z,
    MoveWideOp_K,
}

/// ShiftType
///
/// Generated from the Sail sources at `src/v8_base.sail` L2992.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ShiftType {
    ShiftType_LSL,
    ShiftType_LSR,
    ShiftType_ASR,
    ShiftType_ROR,
}

/// LogicalOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2994.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum LogicalOp {
    LogicalOp_AND,
    LogicalOp_EOR,
    LogicalOp_ORR,
}

/// SystemHintOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L2996-3014.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SystemHintOp {
    SystemHintOp_NOP,
    SystemHintOp_YIELD,
    SystemHintOp_WFE,
    SystemHintOp_WFI,
    SystemHintOp_SEV,
    SystemHintOp_SEVL,
    SystemHintOp_DGH,
    SystemHintOp_ESB,
    SystemHintOp_PSB,
    SystemHintOp_TSB,
    SystemHintOp_BTI,
    SystemHintOp_WFET,
    SystemHintOp_WFIT,
    SystemHintOp_CLRBHB,
    SystemHintOp_GCSB,
    SystemHintOp_CHKFEAT,
    SystemHintOp_CSDB,
}

/// PSTATEField
///
/// Generated from the Sail sources at `src/v8_base.sail` L3016-3030.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PSTATEField {
    PSTATEField_DAIFSet,
    PSTATEField_DAIFClr,
    PSTATEField_PAN,
    PSTATEField_UAO,
    PSTATEField_DIT,
    PSTATEField_SSBS,
    PSTATEField_TCO,
    PSTATEField_SVCRSM,
    PSTATEField_SVCRZA,
    PSTATEField_SVCRSMZA,
    PSTATEField_ALLINT,
    PSTATEField_PM,
    PSTATEField_SP,
}

pub const VMID_NONE: BitVector<16> = Zeros::<16>(16);

/// TLBILevel
///
/// Generated from the Sail sources at `src/v8_base.sail` L3034.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TLBILevel {
    TLBILevel_Any,
    TLBILevel_Last,
}

/// TLBIOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L3036-3061.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TLBIOp {
    TLBIOp_DALL,
    TLBIOp_DASID,
    TLBIOp_DVA,
    TLBIOp_IALL,
    TLBIOp_IASID,
    TLBIOp_IVA,
    TLBIOp_ALL,
    TLBIOp_ASID,
    TLBIOp_IPAS2,
    TLBIPOp_IPAS2,
    TLBIOp_VAA,
    TLBIOp_VA,
    TLBIPOp_VAA,
    TLBIPOp_VA,
    TLBIOp_VMALL,
    TLBIOp_VMALLS12,
    TLBIOp_RIPAS2,
    TLBIPOp_RIPAS2,
    TLBIOp_RVAA,
    TLBIOp_RVA,
    TLBIPOp_RVAA,
    TLBIPOp_RVA,
    TLBIOp_RPA,
    TLBIOp_PAALL,
}

/// TLBIMemAttr
///
/// Generated from the Sail sources at `src/v8_base.sail` L3063.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TLBIMemAttr {
    TLBI_AllAttr,
    TLBI_ExcludeXS,
}

/// TLBIRecord
///
/// Generated from the Sail sources at `src/v8_base.sail` L3065-3081.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLBIRecord {
    pub op: TLBIOp,
    pub from_aarch64: bool,
    pub security: SecurityState,
    pub regime: Regime,
    pub vmid: BitVector<16>,
    pub asid: BitVector<16>,
    pub level: TLBILevel,
    pub attr: TLBIMemAttr,
    pub ipaspace: PASpace,
    pub address: BitVector<64>,
    pub end_address_name: BitVector<64>,
    pub d64: bool,
    pub d128: bool,
    pub ttl: BitVector<4>,
    pub tg: BitVector<2>,
}

pub const MAX_ZERO_BLOCK_SIZE: i128 = 2048;

/// VBitOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L3085.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VBitOp {
    VBitOp_VBIF,
    VBitOp_VBIT,
    VBitOp_VBSL,
    VBitOp_VEOR,
}

/// CompareOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L3087-3093.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CompareOp {
    CompareOp_GT,
    CompareOp_GE,
    CompareOp_EQ,
    CompareOp_LE,
    CompareOp_LT,
}

/// ImmediateOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L3095-3100.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ImmediateOp {
    ImmediateOp_MOVI,
    ImmediateOp_MVNI,
    ImmediateOp_ORR,
    ImmediateOp_BIC,
}

/// ReduceOp
///
/// Generated from the Sail sources at `src/v8_base.sail` L3102-3109.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ReduceOp {
    ReduceOp_FMINNUM,
    ReduceOp_FMAXNUM,
    ReduceOp_FMIN,
    ReduceOp_FMAX,
    ReduceOp_FADD,
    ReduceOp_ADD,
}

/// CrossTriggerIn
///
/// Generated from the Sail sources at `src/v8_base.sail` L3123-3132.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum CrossTriggerIn {
    CrossTriggerIn_CrossHalt,
    CrossTriggerIn_PMUOverflow,
    CrossTriggerIn_RSVD2,
    CrossTriggerIn_RSVD3,
    CrossTriggerIn_TraceExtOut0,
    CrossTriggerIn_TraceExtOut1,
    CrossTriggerIn_TraceExtOut2,
    CrossTriggerIn_TraceExtOut3,
}

pub const DebugHalt_Breakpoint: BitVector<6> = BitVector::<6>::new(0b000111);

pub const DebugHalt_EDBGRQ: BitVector<6> = BitVector::<6>::new(0b010011);

pub const DebugHalt_Step_Normal: BitVector<6> = BitVector::<6>::new(0b011011);

pub const DebugHalt_Step_Exclusive: BitVector<6> = BitVector::<6>::new(0b011111);

pub const DebugHalt_OSUnlockCatch: BitVector<6> = BitVector::<6>::new(0b100011);

pub const DebugHalt_ResetCatch: BitVector<6> = BitVector::<6>::new(0b100111);

pub const DebugHalt_Watchpoint: BitVector<6> = BitVector::<6>::new(0b101011);

pub const DebugHalt_HaltInstruction: BitVector<6> = BitVector::<6>::new(0b101111);

pub const DebugHalt_SoftwareAccess: BitVector<6> = BitVector::<6>::new(0b110011);

pub const DebugHalt_ExceptionCatch: BitVector<6> = BitVector::<6>::new(0b110111);

pub const DebugHalt_Step_NoSyndrome: BitVector<6> = BitVector::<6>::new(0b111011);

/// TLBLine
///
/// Generated from the Sail sources at `src/v8_base.sail` L3156.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLBLine {
    pub tlbrecord: TLBRecord,
    pub valid_name: bool,
}

/// GPTTLBLine
///
/// Generated from the Sail sources at `src/v8_base.sail` L3162.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct GPTTLBLine {
    pub gpt_entry: GPTEntry,
    pub valid_name: bool,
}

pub const RCW64_PROTECTED_BIT: i128 = 52;

pub const RCW128_PROTECTED_BIT: i128 = 114;

/// InterruptReq
///
/// Generated from the Sail sources at `src/v8_base.sail` L3174-3182.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct InterruptReq {
    pub take_SE: bool,
    pub take_vSE: bool,
    pub take_IRQ: bool,
    pub take_vIRQ: bool,
    pub take_FIQ: bool,
    pub take_vFIQ: bool,
    pub iesb_req: bool,
}

/// __InstrEnc
///
/// Generated from the Sail sources at `src/v8_base.sail` L3200.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum __InstrEnc {
    __A64,
    __A32,
    __T16,
    __T32,
}

pub const lst_64bv: BitVector<2> = BitVector::<2>::new(0b01);

pub const lst_64b: BitVector<2> = BitVector::<2>::new(0b10);

pub const lst_64bv0: BitVector<2> = BitVector::<2>::new(0b11);

pub const CFG_ID_AA64PFR0_EL1_EL0: BitVector<4> = BitVector::<4>::new(0b0010);

pub const CFG_ID_AA64PFR0_EL1_EL1: BitVector<4> = BitVector::<4>::new(0b0001);

pub const CFG_ID_AA64PFR0_EL1_EL2: BitVector<4> = BitVector::<4>::new(0b0001);

pub const CFG_ID_AA64PFR0_EL1_EL3: BitVector<4> = BitVector::<4>::new(0b0001);

pub const CFG_PMCR_IDCODE: BitVector<8> = BitVector::<8>::new(0b00000000);

pub const CFG_MPAM_none: BitVector<4> = BitVector::<4>::new(0b0000);

pub const CFG_MPAM_v0p1: BitVector<4> = BitVector::<4>::new(0b0000);

pub const CFG_MPAM_v1p1: BitVector<4> = BitVector::<4>::new(0b0001);

pub const CFG_MPAM_frac_none: BitVector<4> = BitVector::<4>::new(0b0000);

pub const CFG_MPAM_frac_v0p1: BitVector<4> = BitVector::<4>::new(0b0001);

pub const CFG_MPAM_frac_v1p1: BitVector<4> = BitVector::<4>::new(0b0001);

/// SRType
///
/// Generated from the Sail sources at `src/v8_base.sail` L3250.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SRType {
    SRType_LSL,
    SRType_LSR,
    SRType_ASR,
    SRType_ROR,
    SRType_RRX,
}

/// VCGEType
///
/// Generated from the Sail sources at `src/v8_base.sail` L3252.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VCGEType {
    VCGEType_signed,
    VCGEType_unsigned,
    VCGEType_fp,
}

/// VFPNegMul
///
/// Generated from the Sail sources at `src/v8_base.sail` L3254.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VFPNegMul {
    VFPNegMul_VNMLA,
    VFPNegMul_VNMLS,
    VFPNegMul_VNMUL,
}

/// VCGTtype
///
/// Generated from the Sail sources at `src/v8_base.sail` L3256.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VCGTtype {
    VCGTtype_signed,
    VCGTtype_unsigned,
    VCGTtype_fp,
}

/// VBitOps
///
/// Generated from the Sail sources at `src/v8_base.sail` L3258.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VBitOps {
    VBitOps_VBIF,
    VBitOps_VBIT,
    VBitOps_VBSL,
}

pub const DebugException_Breakpoint: BitVector<4> = BitVector::<4>::new(0b0001);

pub const DebugException_BKPT: BitVector<4> = BitVector::<4>::new(0b0011);

pub const DebugException_VectorCatch: BitVector<4> = BitVector::<4>::new(0b0101);

pub const DebugException_Watchpoint: BitVector<4> = BitVector::<4>::new(0b1010);

pub type CNTPOFF_EL2_Type = BitVector<64>;

pub type CNTVOFF_EL2_Type = BitVector<64>;

pub type DLR_EL0_Type = BitVector<64>;

pub type DLR_Type = BitVector<32>;

pub type SP_EL0_Type = BitVector<64>;

pub type SP_EL1_Type = BitVector<64>;

pub type SP_EL2_Type = BitVector<64>;

pub type SP_EL3_Type = BitVector<64>;

/// X_set
///
/// Generated from the Sail sources at `src/v8_base.sail` L7434-7441.
pub fn X_set<const WIDTH: i128>(
    core_ctx: &mut Core,
    n: i128,
    width: i128,
    value_name: BitVector<WIDTH>,
) {
    assert!(((n >= 0) && (n <= 31)), "src/v8_base.sail:7435.27-7435.28");
    assert!(
        ((width == 32) || (width == 64)),
        "src/v8_base.sail:7436.36-7436.37"
    );
    if { (n != 31) } {
        set_R(core_ctx, n, ZeroExtend0(value_name, 64))
    } else {
        ()
    };
    return ();
}

pub type FAR_EL1_Type = BitVector<64>;

pub type FAR_EL2_Type = BitVector<64>;

pub type FAR_EL3_Type = BitVector<64>;

pub type ELR_EL1_Type = BitVector<64>;

pub type ELR_EL2_Type = BitVector<64>;

pub type ELR_EL3_Type = BitVector<64>;

pub type VBAR_EL1_Type = BitVector<64>;

pub type VBAR_EL2_Type = BitVector<64>;

pub type VBAR_EL3_Type = BitVector<64>;

pub type VBAR_Type = BitVector<32>;

pub const TAG_GRANULE: i128 = 16;

/// AddWithCarry
///
/// Generated from the Sail sources at `src/v8_base.sail` L13337-13346.
pub fn AddWithCarry<const N: i128>(
    x: BitVector<N>,
    y: BitVector<N>,
    carry_in: BitVector<1>,
) -> (BitVector<N>, BitVector<4>) {
    let unsigned_sum = ((UInt0(x) + UInt0(y)) + UInt0(carry_in));
    let signed_sum = ((SInt0(x) + SInt0(y)) + UInt0(carry_in));
    let result: BitVector<N> = integer_subrange(unsigned_sum, (bitvector_length(y) - 1), 0);
    let n: BitVector<1> =
        BitVector::new(0).set_bit(0, bitvector_access(result, (bitvector_length(y) - 1)));
    let z: BitVector<1> = if { IsZero(result) } {
        BitVector::<1>::new(0b1)
    } else {
        BitVector::<1>::new(0b0)
    };
    let c: BitVector<1> = if { (UInt0(result) == unsigned_sum) } {
        BitVector::<1>::new(0b0)
    } else {
        BitVector::<1>::new(0b1)
    };
    let v: BitVector<1> = if { (SInt0(result) == signed_sum) } {
        BitVector::<1>::new(0b0)
    } else {
        BitVector::<1>::new(0b1)
    };
    return (
        result,
        bitvector_concat::<3, 1, 4>(
            bitvector_concat::<2, 1, 3>(bitvector_concat::<1, 1, 2>(n, z), c),
            v,
        ),
    );
}

pub type HDFAR_Type = BitVector<32>;

pub type HIFAR_Type = BitVector<32>;

pub type ELR_hyp_Type = BitVector<32>;

pub type HVBAR_Type = BitVector<32>;

pub type DFAR_Type = BitVector<32>;

pub type IFAR_Type = BitVector<32>;

pub const GIC_PENDING_NONE: BitVector<32> =
    BitVector::<32>::new(0b00000000000000000000001111111111);

/// arm_acc_type
///
/// Generated from the Sail sources at `src/interface.sail` L49-72.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum arm_acc_type {
    SAcc_ASIMD(bool),
    SAcc_SVE(bool),
    SAcc_SME(bool),
    SAcc_IC(()),
    SAcc_DC(()),
    SAcc_DCZero(()),
    SAcc_AT(()),
    SAcc_NV2(()),
    SAcc_SPE(()),
    SAcc_GCS(()),
    SAcc_GPTW(()),
}

pub type APIAKeyHi_EL1_Type = BitVector<64>;

pub type APIAKeyLo_EL1_Type = BitVector<64>;

pub type APIBKeyHi_EL1_Type = BitVector<64>;

pub type APIBKeyLo_EL1_Type = BitVector<64>;

pub type APDAKeyHi_EL1_Type = BitVector<64>;

pub type APDAKeyLo_EL1_Type = BitVector<64>;

pub type APDBKeyHi_EL1_Type = BitVector<64>;

pub type APDBKeyLo_EL1_Type = BitVector<64>;

pub type APGAKeyHi_EL1_Type = BitVector<64>;

pub type APGAKeyLo_EL1_Type = BitVector<64>;

/// X_read
///
/// Generated from the Sail sources at `src/v8_base.sail` L22318-22326.
pub fn X_read<const WIDTH: i128>(core_ctx: &mut Core, n: i128, width: i128) -> BitVector<WIDTH> {
    assert!(
        ((n >= 0) && (n <= 31)),
        "src/v8_base.sail:22319.27-22319.28"
    );
    assert!(
        ((width == 8) || ((width == 16) || ((width == 32) || (width == 64)))),
        "src/v8_base.sail:22320.63-22320.64"
    );
    if { (n != 31) } {
        return subrange_bits(get_R(core_ctx, n), (width - 1), 0);
    } else {
        return Zeros(width);
    }
}

pub type EDWAR_Type = BitVector<64>;

pub type CTIDEVID1_Type = BitVector<32>;

pub type CTIDEVID2_Type = BitVector<32>;

pub type DBGDEVID2_Type = BitVector<32>;

pub type DBGDSAR_Type = BitVector<64>;

pub type DBGWFAR_Type = BitVector<32>;

pub type EDDEVID2_Type = BitVector<32>;

pub type FCSEIDR_Type = BitVector<32>;

pub type JIDR_Type = BitVector<32>;

pub type JMCR_Type = BitVector<32>;

pub type JOSCR_Type = BitVector<32>;

pub type ID_AFR0_EL1_Type = BitVector<64>;

pub type ID_AFR0_Type = BitVector<32>;

pub type CNTFRQ_EL0_Type = BitVector<64>;

pub type DBGDTRRX_EL0_Type = BitVector<64>;

pub type DBGDTRTX_EL0_Type = BitVector<64>;

pub type HFGITR2_EL2_Type = BitVector<64>;

pub type ID_AA64AFR0_EL1_Type = BitVector<64>;

pub type ID_AA64AFR1_EL1_Type = BitVector<64>;

pub type OSDTRRX_EL1_Type = BitVector<64>;

pub type OSDTRTX_EL1_Type = BitVector<64>;

pub type PMEVCNTR_EL0_Type = BitVector<64>;

/// TLBIInfo
///
/// Generated from the Sail sources at `src/interface.sail` L156-159.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLBIInfo {
    pub rec: TLBIRecord,
    pub shareability: Shareability,
}

pub type PMEVCNTR_Type = BitVector<32>;

pub type ICC_AP0R_EL1_Type = BitVector<64>;

pub type AIDR_EL1_Type = BitVector<64>;

pub type REVIDR_EL1_Type = BitVector<64>;

pub type TPIDR_EL2_Type = BitVector<64>;

pub type ACTLR_EL2_Type = BitVector<64>;

pub type TPIDR_EL1_Type = BitVector<64>;

pub type ACTLR_EL1_Type = BitVector<64>;

pub type AFSR1_EL1_Type = BitVector<64>;

pub type AMAIR2_EL2_Type = BitVector<64>;

pub type ICV_AP0R_EL1_Type = BitVector<64>;

pub type AFSR0_EL3_Type = BitVector<64>;

pub type AFSR1_EL2_Type = BitVector<64>;

pub type AFSR0_EL1_Type = BitVector<64>;

pub type SCXTNUM_EL2_Type = BitVector<64>;

pub type TPIDR_EL3_Type = BitVector<64>;

pub type ACTLR_EL3_Type = BitVector<64>;

pub type AMAIR_EL2_Type = BitVector<64>;

pub type AMAIR_EL3_Type = BitVector<64>;

pub type SCXTNUM_EL1_Type = BitVector<64>;

pub type TPIDRRO_EL0_Type = BitVector<64>;

pub type AMAIR_EL1_Type = BitVector<64>;

pub type SCXTNUM_EL0_Type = BitVector<64>;

pub type TPIDR2_EL0_Type = BitVector<64>;

pub type SCXTNUM_EL3_Type = BitVector<64>;

pub type TPIDR_EL0_Type = BitVector<64>;

pub type HACR_EL2_Type = BitVector<64>;

pub type AMAIR2_EL3_Type = BitVector<64>;

pub type AFSR1_EL3_Type = BitVector<64>;

pub type AFSR0_EL2_Type = BitVector<64>;

pub type AMAIR2_EL1_Type = BitVector<64>;

pub type CNTVOFF_Type = BitVector<64>;

pub type AMEVCNTVOFF0_EL2_Type = BitVector<64>;

pub type AMEVCNTVOFF1_EL2_Type = BitVector<64>;

pub type ERXADDR_EL1_Type = BitVector<64>;

pub type ERXMISC0_EL1_Type = BitVector<64>;

pub type ERXPFGCDN_EL1_Type = BitVector<64>;

pub type ERXPFGF_EL1_Type = BitVector<64>;

pub type ERXMISC1_EL1_Type = BitVector<64>;

pub type ERXCTLR_EL1_Type = BitVector<64>;

pub type ERXSTATUS_EL1_Type = BitVector<64>;

pub type ERXFR_EL1_Type = BitVector<64>;

pub type ERXMISC3_EL1_Type = BitVector<64>;

pub type ERXMISC2_EL1_Type = BitVector<64>;

pub type ERXPFGCTL_EL1_Type = BitVector<64>;

pub type AIFSR_Type = BitVector<32>;

pub type HACR_Type = BitVector<32>;

pub type HACTLR_Type = BitVector<32>;

pub type TCMTR_Type = BitVector<32>;

pub type ICV_AP0R_Type = BitVector<32>;

pub type TPIDRURW_Type = BitVector<32>;

pub type ICV_AP1R_Type = BitVector<32>;

pub type ADFSR_Type = BitVector<32>;

pub type REVIDR_Type = BitVector<32>;

pub type TPIDRURO_Type = BitVector<32>;

pub type ACTLR_Type = BitVector<32>;

pub type ICC_AP1R_Type = BitVector<32>;

pub type AMAIR0_Type = BitVector<32>;

pub type HAMAIR0_Type = BitVector<32>;

pub type AIDR_Type = BitVector<32>;

pub type HADFSR_Type = BitVector<32>;

pub type CNTFRQ_Type = BitVector<32>;

pub type ACTLR2_Type = BitVector<32>;

pub type HAMAIR1_Type = BitVector<32>;

pub type HAIFSR_Type = BitVector<32>;

pub type ICC_AP0R_Type = BitVector<32>;

pub type TPIDRPRW_Type = BitVector<32>;

pub type HTPIDR_Type = BitVector<32>;

pub type AMAIR1_Type = BitVector<32>;

pub type ERXFR2_Type = BitVector<32>;

pub type ERXMISC2_Type = BitVector<32>;

pub type ERXFR_Type = BitVector<32>;

pub type ERXADDR_Type = BitVector<32>;

pub type ERXMISC0_Type = BitVector<32>;

pub type ERXMISC5_Type = BitVector<32>;

pub type ERXCTLR2_Type = BitVector<32>;

pub type ERXMISC1_Type = BitVector<32>;

pub type ERXCTLR_Type = BitVector<32>;

pub type ERXMISC6_Type = BitVector<32>;

pub type ERXMISC4_Type = BitVector<32>;

pub type ERXADDR2_Type = BitVector<32>;

pub type ERXMISC7_Type = BitVector<32>;

pub type ERXMISC3_Type = BitVector<32>;

pub type ERXSTATUS_Type = BitVector<32>;

pub type HACTLR2_Type = BitVector<32>;

/// DxB
///
/// Generated from the Sail sources at `src/interface.sail` L167-171.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct DxB {
    pub domain: MBReqDomain,
    pub types: MBReqTypes,
    pub nXS: bool,
}

/// Barrier
///
/// Generated from the Sail sources at `src/interface.sail` L174-181.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Barrier {
    Barrier_DSB(DxB),
    Barrier_DMB(DxB),
    Barrier_ISB(()),
    Barrier_SSBB(()),
    Barrier_PSSBB(()),
    Barrier_SB(()),
}

/// execute_aarch64_instrs_integer_arithmetic_add_sub_carry
///
/// Generated from the Sail sources at `src/instrs64.sail` L194-207.
pub fn execute_aarch64_instrs_integer_arithmetic_add_sub_carry<const DATASIZE: i128>(
    core_ctx: &mut Core,
    d: i128,
    datasize: i128,
    m: i128,
    n: i128,
    setflags: bool,
    sub_op: bool,
) {
    let mut result: BitVector<DATASIZE> = undefined_bitvector(__id(datasize));
    {
        let operand1: BitVector<DATASIZE> = X_read(core_ctx, n, datasize);
        let mut operand2: BitVector<DATASIZE> = X_read(core_ctx, m, datasize);
        {
            let mut nzcv: BitVector<4> = undefined_bitvector::<4>(4);
            {
                if { sub_op } {
                    operand2 = !(operand2)
                } else {
                    ()
                };
                let (tup__0, tup__1) = ({
                    let var_1 = core_ctx.PSTATE.C;
                    AddWithCarry(operand1, operand2, var_1)
                } as (BitVector<DATASIZE>, BitVector<4>));
                result = tup__0;
                nzcv = tup__1;
                if { setflags } {
                    {
                        core_ctx.PSTATE.N = nzcv.subrange::<3, 4, 1>();
                        core_ctx.PSTATE.Z = nzcv.subrange::<2, 3, 1>();
                        core_ctx.PSTATE.C = nzcv.subrange::<1, 2, 1>();
                        core_ctx.PSTATE.V = nzcv.subrange::<0, 1, 1>()
                    }
                } else {
                    ()
                };
                X_set(core_ctx, d, datasize, result)
            }
        }
    }
}

pub const COLD_RESET: bool = true;
