Get Conda
conda env create -f environment.yml
conda activate tch-rs-arm
ln -sf /opt/homebrew/Caskroom/miniconda/base/envs/tch-rs-arm/lib/python3.12/site-packages/torch torch
