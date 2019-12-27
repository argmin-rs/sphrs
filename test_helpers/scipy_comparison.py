"""
Computes a range of spherical harmonics using `sph_harm` of `scipy.special` and prints it
to the terminal as CSV.
"""

from math import pi
from scipy.special import sph_harm


def main():
    """
    Computes a range of spherical harmonics for comparison
    """
    num_phis = 10
    num_thetas = 2 * num_phis
    thetas = [2 * pi / num_thetas * x for x in list(range(0, num_thetas, 1))]
    phis = [pi / num_phis * x for x in list(range(0, num_phis, 1))]
    n_max = 10
    print("n,m,theta,phi,sph_re,sph_im")
    for n in range(0, n_max + 1):
        for m in range(-n, n + 1):
            for phi in phis:
                for theta in thetas:
                    res = sph_harm(m, n, theta, phi)
                    print(
                        "{},{},{},{},{},{}".format(n, m, theta, phi, res.real, res.imag)
                    )


if __name__ == "__main__":
    main()
