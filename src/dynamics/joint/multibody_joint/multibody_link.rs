use std::ops::{Deref, DerefMut};

use crate::dynamics::{MultibodyJoint, RigidBodyHandle};
use crate::math::{Isometry, Real, Vector};
use crate::prelude::RigidBodyVelocity;

/// One link of a multibody.
pub struct MultibodyLink {
    // FIXME: make all those private.
    pub(crate) internal_id: usize,
    pub(crate) assembly_id: usize,

    pub(crate) parent_internal_id: usize,
    pub(crate) rigid_body: RigidBodyHandle,

    /*
     * Change at each time step.
     */
    pub joint: MultibodyJoint,
    // TODO: should this be removed in favor of the rigid-body position?
    pub local_to_world: Isometry<Real>,
    pub local_to_parent: Isometry<Real>,
    pub shift02: Vector<Real>,
    pub shift23: Vector<Real>,

    /// The velocity added by the joint, in world-space.
    pub(crate) joint_velocity: RigidBodyVelocity,
}

impl MultibodyLink {
    /// Creates a new multibody link.
    pub fn new(
        rigid_body: RigidBodyHandle,
        internal_id: usize,
        assembly_id: usize,
        parent_internal_id: usize,
        joint: MultibodyJoint,
        local_to_world: Isometry<Real>,
        local_to_parent: Isometry<Real>,
    ) -> Self {
        let joint_velocity = RigidBodyVelocity::zero();

        MultibodyLink {
            internal_id,
            assembly_id,
            parent_internal_id,
            joint,
            local_to_world,
            local_to_parent,
            shift02: na::zero(),
            shift23: na::zero(),
            joint_velocity,
            rigid_body,
        }
    }

    pub fn joint(&self) -> &MultibodyJoint {
        &self.joint
    }

    pub fn rigid_body_handle(&self) -> RigidBodyHandle {
        self.rigid_body
    }

    /// Checks if this link is the root of the multibody.
    #[inline]
    pub fn is_root(&self) -> bool {
        self.internal_id == 0
    }

    /// The handle of this multibody link.
    #[inline]
    pub fn link_id(&self) -> usize {
        self.internal_id
    }

    /// The handle of the parent link.
    #[inline]
    pub fn parent_id(&self) -> Option<usize> {
        if self.internal_id != 0 {
            Some(self.parent_internal_id)
        } else {
            None
        }
    }

    #[inline]
    pub fn local_to_world(&self) -> &Isometry<Real> {
        &self.local_to_world
    }

    #[inline]
    pub fn local_to_parent(&self) -> &Isometry<Real> {
        &self.local_to_parent
    }
}

// FIXME: keep this even if we already have the Index2 traits?
pub(crate) struct MultibodyLinkVec(pub Vec<MultibodyLink>);

impl MultibodyLinkVec {
    #[inline]
    pub fn get_mut_with_parent(&mut self, i: usize) -> (&mut MultibodyLink, &MultibodyLink) {
        let parent_id = self[i].parent_internal_id;

        assert!(
            parent_id != i,
            "Internal error: circular rigid body dependency."
        );
        assert!(parent_id < self.len(), "Invalid parent index.");

        unsafe {
            let rb = &mut *(self.get_unchecked_mut(i) as *mut _);
            let parent_rb = &*(self.get_unchecked(parent_id) as *const _);
            (rb, parent_rb)
        }
    }
}

impl Deref for MultibodyLinkVec {
    type Target = Vec<MultibodyLink>;

    #[inline]
    fn deref(&self) -> &Vec<MultibodyLink> {
        let MultibodyLinkVec(ref me) = *self;
        me
    }
}

impl DerefMut for MultibodyLinkVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Vec<MultibodyLink> {
        let MultibodyLinkVec(ref mut me) = *self;
        me
    }
}
