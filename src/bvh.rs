use crate::{
    aabb::AABB,
    hit::{HitRange, HitRec, Hittable},
    material::Material,
    ray::Ray,
};

enum BvhNode<'a, T: Hittable + Material> {
    Tree(usize, usize, AABB),
    Leaf(&'a T),
}

pub struct Bvh<'a, T: Hittable + Material> {
    nodes: Vec<BvhNode<'a, T>>,
}

fn build_rec<'a, T: Hittable + Material>(nodes: &mut Vec<BvhNode<'a, T>>, hittables: &'a mut [T]) {
    let bbox = hittables.bounding_box();
    match hittables.len() {
        0 => panic!("cannot be zero."),
        1 => {
            let root = BvhNode::Tree(nodes.len() + 1, nodes.len() + 1, bbox);
            let l = BvhNode::Leaf(&hittables[0]);
            nodes.push(root);
            nodes.push(l);
        }
        2 => {
            let root = BvhNode::Tree(nodes.len() + 1, nodes.len() + 2, bbox);
            let l = BvhNode::Leaf(&hittables[0]);
            let r = BvhNode::Leaf(&hittables[1]);
            nodes.push(root);
            nodes.push(l);
            nodes.push(r);
        }
        _ => {
            let axis_cmp = hittables.bounding_box().longest_axis();
            hittables.sort_unstable_by_key(|h| h.bounding_box()[axis_cmp].start as u64);
            let midpoint = hittables.len() / 2;
            let (l, r) = hittables.split_at_mut(midpoint);
            let root_idx = nodes.len();
            let root = BvhNode::Tree(nodes.len() + 1, 0, bbox);
            nodes.push(root);
            build_rec(nodes, l);
            let right_idx = nodes.len();
            match &mut nodes[root_idx] {
                BvhNode::Tree(_, right, _) => *right = right_idx,
                _ => panic!("bad index"),
            }
            build_rec(nodes, r);
        }
    }
}

impl<'a, T: Hittable + Material> Bvh<'a, T> {
    pub fn new(hittables: &'a mut [T]) -> Self {
        let mut nodes = vec![];
        build_rec(&mut nodes, hittables);
        Self { nodes }
    }
}

impl<'a, T: Hittable + Material> Hittable for Bvh<'a, T> {
    fn hit(&self, r: &Ray, t_lim: &HitRange) -> Option<HitRec> {
        let mut dfs = vec![(&self.nodes[0], 0, *t_lim)];
        let mut tmax = t_lim.end;
        let mut ret = None;
        while let Some((node, idx, t_lim)) = dfs.pop() {
            match node {
                BvhNode::Tree(left, right, aabb) => {
                    let Some(t_lim) = aabb.intersects(r, &t_lim) else {
                        continue;
                    };
                    dfs.push((&self.nodes[*left], *left, t_lim));
                    dfs.push((&self.nodes[*right], *right, t_lim));
                }
                BvhNode::Leaf(h) => {
                    if let Some(hr) = h
                        .hit(r, &(t_lim.start..tmax).into())
                        .map(|h| h.set_ancillary(idx))
                    {
                        tmax = hr.t;
                        ret = Some(hr);
                    }
                }
            }
        }
        ret
    }

    fn bounding_box(&self) -> AABB {
        match &self.nodes[0] {
            BvhNode::Tree(_, _, aabb) => aabb.clone(),
            BvhNode::Leaf(_) => unreachable!(),
        }
    }
}

impl<'a, T: Hittable + Material> Material for Bvh<'a, T> {
    fn scatter(&self, ray: &Ray, hit: &HitRec) -> Option<crate::material::Scatter> {
        match self.nodes[hit.ancillary] {
            BvhNode::Leaf(m) => m.scatter(ray, hit),
            _ => {
                eprintln!("ERROR: Should not have matched a tree node in Bvh::scatter()");
                None
            }
        }
    }
}
