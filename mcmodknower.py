"""Docstring."""
import cProfile
import os
from semantic_digital_twin.adapters.urdf import URDFParser
from semantic_digital_twin.reasoning.predicates import is_supported_by
from semantic_digital_twin.reasoning.world_reasoner import WorldReasoner
from semantic_digital_twin.world import World
from semantic_digital_twin.semantic_annotations.mixins import HasSupportingSurface
import time

# before running this for the first time,
# ensure all dependencies are installed/built
setup_commands = """
sudo apt install -y graphviz graphviz-dev
pip install mujoco giskardpy_bullet_bindings
pip install --editable ./semantic_digital_twin ./krrood ./giskardpy
"""

output = None  # or open("output-cram.txt","x")

os.environ['ROS_PACKAGE_PATH'] = "../iai_maps/iai_apartment"
file_name = "package://iai_apartment/urdf/apartment.urdf"
# file_name = "../../latex/program/program/fixed.urdf"
# file_name = "../../latex/program/gz_robocup_2021_2.urdf"

print('parsing urdf')
world: World = URDFParser.from_file(file_name).parse()
print('reasoning')
world_reasoner = WorldReasoner(world)
world_reasoner.reason()
bodies = world.bodies_with_collision

# print(world.semantic_annotations)

# for b in world.semantic_annotations:
#     if (isinstance(b, HasSupportingSurface)):
#         print('Has Supporting Surface')
#         print(b)
# exit(1)

def test():
    print(f'testing supported by for {len(bodies)} bodies')
    n = 0
    p = 1024
    total = len(bodies)**2
    t = 0
    start = time.clock_gettime_ns(time.CLOCK_BOOTTIME)
    for b1 in bodies:
        for b2 in bodies:
            if is_supported_by(b1, b2):
                t += 1
                if output:
                    output.write(f'is_supported_by({b1.name}, {b2.name})\n')
            n += 1
            if n >= p:
                mid = time.clock_gettime_ns(time.CLOCK_BOOTTIME)
                print(f'tested {n} relations, with {t} being true')
                print(f'elapsed: {mid-start} ns, expected end in {(mid-start)*total/n - (mid-start)} ns')
                p += 1024

    end = time.clock_gettime_ns(time.CLOCK_BOOTTIME)
    print(f'tested {n} relations, with {t} being true.')
    elapsed = end-start
    print(f'finished after {elapsed} ns (aka {elapsed/1e9} s)')
    print(f'average time per test: {elapsed / (n*1e6)} ms')


# cProfile.run('test()', "test.prof")
test()
