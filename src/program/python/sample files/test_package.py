import pkg_resources

package_set: pkg_resources.WorkingSet = pkg_resources.working_set

if 'pycrypto' in package_set.by_key:
    print('installed')
else:
    print('not installed')


try:
    from pip._internal.utils.misc import get_installed_distributions
except ImportError:  # pip<10
    from pip import get_installed_distributions

dists = get_installed_distributions()
for d in dists:
    print(d.project_name)
